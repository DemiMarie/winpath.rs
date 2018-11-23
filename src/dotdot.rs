use std::usize;
const BACKSLASH: u16 = b'\\' as _;
const SLASH: u16 = b'/' as _;
const QUESTION_MARK: u16 = b'?' as _;
const PERIOD: u16 = b'.' as _;
const VERBATIM_PREFIX: &'static [u16] = &[BACKSLASH, BACKSLASH, QUESTION_MARK, BACKSLASH];
const LOCAL_DEVICE: &'static [u16] = &[BACKSLASH, BACKSLASH, PERIOD, BACKSLASH];
const VERBATIM_UNC_PREFIX: &'static [u16] = &[
    BACKSLASH,
    BACKSLASH,
    QUESTION_MARK,
    BACKSLASH,
    b'U' as _,
    b'N' as _,
    b'C' as _,
    BACKSLASH,
];
const COLON: u16 = b':' as _;
const C_DRIVE: &'static [u16] = &[b'C' as _, b':' as _];
fn is_sep(arg: u16) -> bool {
    arg == BACKSLASH || arg == SLASH
}

fn is_sep_ref(arg: &u16) -> bool {
    is_sep(*arg)
}

fn case_insensitive_eq(wchar: &[u16], bytes: &[u8]) -> bool {
    let len = bytes.len();
    if wchar.len() != len {
        return false;
    }
    for i in 0..len {
        if (wchar[i] | 0x20) != (<u8 as Into<u16>>::into(bytes[i]) | 0x20) {
            return false;
        }
    }
    true
}

fn is_pipe(obj: &[u16]) -> bool {
    case_insensitive_eq(obj, b"pipe")
}

fn is_unc(obj: &[u16]) -> bool {
    case_insensitive_eq(obj, b"unc")
}

fn is_redirect(obj: &[u16]) -> bool {
    obj.get(0) == Some(&(b';'.into()))
        && (case_insensitive_eq(&obj[1..], b"LanManRedirector")
            || case_insensitive_eq(&obj[1..], b"WebDavRedirector"))
}

fn make_pipe_path(prefix: &[u16], buf: &[u16], off: usize) -> Vec<u16> {
    assert!(buf.len() >= off);
    let mut res = Vec::with_capacity(buf.len() + prefix.len());
    res.extend_from_slice(prefix);
    res.extend_from_slice(buf);
    for i in 0..off {
        if (b'/' as u16) == buf[i] {
            res[i + prefix.len()] = b'\\'.into();
        }
    }
    res
}

fn split_first_part(buf: &[u16]) -> Result<(&[u16], &[u16]), ()> {
    match buf.iter().position(is_sep_ref) {
        None => Err(()),
        Some(x) => Ok((&buf[..x], &buf[x..])),
    }
}

fn split_first_part_disallow_empty(buf: &[u16]) -> Result<(&[u16], &[u16]), ()> {
    match buf.iter().position(is_sep_ref) {
        None => Err(()),
        Some(0) => Err(()),
        Some(x) => Ok((&buf[..x], &buf[x..])),
    }
}

fn process_device_path(buf: &[u16]) -> Result<Vec<u16>, ()> {
    fn replace_prefix(buf: &[u16]) -> Vec<u16> {
        let mut v = Vec::with_capacity(buf.len());
        v.extend_from_slice(&VERBATIM_PREFIX);
        v.extend_from_slice(&buf[4..]);
        v
    }

    let (srv, suffix) = match split_first_part_disallow_empty(&buf[4..]) {
        Ok(x) => x,
        Err(()) => {
            return match buf.get(4) {
                None => Err(()),
                Some(&x) if is_sep(x) => Err(()),
                Some(_) => Ok(replace_prefix(buf)),
            }
        }
    };

    if is_pipe(srv) {
        // Pipe path (verbatim)
        let mut res = replace_prefix(buf);
        res.get_mut(8).map(|x| {
            if *x == '/' as _ {
                *x = BACKSLASH
            }
        });
        Ok(res)
    } else if is_unc(srv) {
        // UNC path
        split_srv_and_share(&buf[8..])
    } else {
        // eprintln!("Server: {:?} of len {:?}", srv, srv.len());
        Ok(remove_dotdot(buf, 1 + srv.len(), &VERBATIM_PREFIX).0)
    }
}

fn check_unc_pipe_path(buf: &[u16], srv: &[u16], share: &[u16], offset: usize) -> Vec<u16> {
    if is_pipe(share) {
        // Named pipe file system (NPFS) path.  THESE PATHS ARE VERBATIM!!!
        // 24 == br"\;LanManRedirector\pipe\".len() &&
        // 24 == br"\;WebDavRedirector\pipe\".len()
        make_pipe_path(VERBATIM_UNC_PREFIX, buf, offset + 8 + srv.len())
    } else {
        remove_dotdot(
            buf,
            offset + srv.len() + share.len(),
            &VERBATIM_UNC_PREFIX[..],
        ).0
    }
}

pub fn split_srv_and_share(buf: &[u16]) -> Result<Vec<u16>, ()> {
    let (srv, suffix) = split_first_part_disallow_empty(buf)?;
    if suffix.len() < 1 {
        return Err(());
    }
    let (share, suffix) = split_first_part(&suffix[1..])?;
    if share.is_empty() {
        Err(())
    } else if is_redirect(share) {
        // `;LanManRedirector` or `;WebDavRedirector` path
        let (real_share, _suffix) = split_first_part(suffix)?;
        if real_share.is_empty() {
            Err(())
        } else {
            Ok(check_unc_pipe_path(buf, srv, real_share, 20))
        }
    } else {
        Ok(check_unc_pipe_path(buf, srv, share, 2))
    }
}

mod the_trait {
    pub trait Conv {
        type Output;
        fn conv<T, U>(self, cb: T) -> U
        where
            T: FnOnce(&[u16]) -> U;
        fn rev_conv(buf: Vec<u16>) -> Self::Output;
    }
    impl<'a> Conv for &'a [u16] {
        type Output = Vec<u16>;
        fn conv<T, U>(self, cb: T) -> U
        where
            T: FnOnce(&[u16]) -> U,
        {
            cb(self)
        }
        fn rev_conv(buf: Vec<u16>) -> Self::Output {
            buf
        }
    }
    impl<'a> Conv for &'a str {
        type Output = String;
        fn conv<T, U>(self, cb: T) -> U
        where
            T: FnOnce(&[u16]) -> U,
        {
            let vec: Vec<_> = self.encode_utf16().collect();
            cb(&vec)
        }
        fn rev_conv(buf: Vec<u16>) -> Self::Output {
            String::from_utf16(&buf).unwrap()
        }
    }
}

pub fn to_u16<T>(orig_path: T) -> Result<Vec<u16>, ()>
where
    T: the_trait::Conv,
{
    orig_path.conv(|x| Ok(x.to_owned()))
}

#[cfg(windows)]
mod windows {
    use winapi::um::processenv::GetCurrentDirectoryW;
    pub fn get_current_directory() -> Vec<u16> {
        let mut buf_size = 257;
        let mut v: Vec<u16> = Vec::with_capacity(buf_size);
        loop {
            let mut q;
            unsafe {
                q = GetCurrentDirectoryW(v.capacity() as _, v.as_ptr());
                if q > 0 {
                    debug_assert!(q < v.capacity());
                    v.set_len(q);
                    return v;
                } else {
                    panic!("bug");
                }
            }
            v.reserve(q + 1);
        }
    }
}

#[cfg(not(windows))]
mod windows {
    use dotdot::the_trait::Conv;
    pub fn get_current_directory() -> Vec<u16> {
        ("C:\\alpha").conv(|x| x.to_owned())
    }
}

pub fn to_canon_path<T>(orig_path: T) -> Result<T::Output, ()>
where
    T: the_trait::Conv,
{
    use self::windows::get_current_directory;
    orig_path
        .conv(|x| {
            let start = match x.get(0) {
                None => return Err(()),
                Some(q) => *q,
            };
            if is_sep(start) {
                match x.get(1) {
                    Some(63) if x.len() >= 4 && x[2] == 63 && is_sep(x[3]) => {
                        // \??\
                        Ok(make_pipe_path(VERBATIM_PREFIX, &x[4..], 0))
                    }
                    Some(92) | Some(47) => if x.len() < 4 {
                        // Too short
                        Err(())
                    } else if x[2] == b'.' as _ && is_sep(x[3]) {
                        // \\.\
                        process_device_path(x)
                    } else if x[2] == (b'?' as _) && is_sep(x[3]) {
                        // \\?\
                        Ok(make_pipe_path(VERBATIM_PREFIX, &x[4..], 0))
                    } else {
                        split_srv_and_share(&x[2..])
                    },
                    None | Some(_) => {
                        let mut q = get_current_directory();
                        if q.len() < 2 || q[1] != COLON {
                            Err(())
                        } else {
                            q.resize(2, 0);
                            q.extend_from_slice(&x);
                            to_canon_path(&q[..])
                        }
                    }
                }
            } else if x.len() >= 2 && x[1] == COLON {
                if x.len() == 2 {
                    let mut res = Vec::with_capacity(7);
                    res.extend_from_slice(VERBATIM_PREFIX);
                    res.extend_from_slice(x);
                    res.push(BACKSLASH);
                    Ok(res)
                } else if is_sep(x[2]) {
                    let mut s = remove_dotdot(x, 3, VERBATIM_PREFIX).0;
                    s[6] = BACKSLASH;
                    Ok(s)
                } else {
                    let mut cwd = get_current_directory();
                    if cwd.len() > 2 && cwd[..2] == x[..2] {
                        if Some(&BACKSLASH) != cwd.last() {
                            cwd.push(BACKSLASH)
                        }
                        cwd.extend_from_slice(&x[2..]);
                        to_canon_path(&cwd[..])
                    } else {
                        Err(())
                    }
                }
            } else {
                let mut cwd = get_current_directory();
                if Some(&BACKSLASH) != cwd.last() {
                    cwd.push(BACKSLASH)
                }
                cwd.extend_from_slice(&x);
                to_canon_path(&cwd[..])
            }
        }).map(T::rev_conv)
}

pub fn remove_dotdot(buf: &[u16], mut prefix_len: usize, prefix_slice: &[u16]) -> (Vec<u16>, bool) {
    assert!(buf.len() >= prefix_len);
    assert!(prefix_slice.len() + buf.len() <= usize::MAX);
    let mut output = vec![];
    output.extend_from_slice(prefix_slice);
    prefix_len += prefix_slice.len();
    let mut underflow = false;
    for i in buf.split(is_sep_ref) {
        match i {
            &[46] | &[] => {}
            &[46, 46] if output.len() > prefix_len => {
                let mut seen_non_backslash = false;
                loop {
                    match output.pop() {
                        Some(92) if seen_non_backslash || output.len() == prefix_len => break,
                        Some(92) => {}
                        None => {
                            underflow = true;
                            break;
                        }
                        _ => seen_non_backslash = true,
                    }
                }
            }
            &[46, 46] => {}
            x => {
                if !x.is_empty() && output.len() > prefix_slice.len() {
                    output.push(b'\\' as _);
                }
                output.extend_from_slice(x);
            }
        }
    }
    if output.len() < prefix_len {
        output.push(BACKSLASH);
        // panic!("output: {:?}, prefix_len: {:?}", output, prefix_len)
    }
    (
        if output.is_empty() {
            vec!['\\' as _]
        } else {
            for i in prefix_slice.len()..prefix_len {
                if output[i] == b'/' as _ {
                    output[i] = b'\\' as _
                }
            }
            output
        },
        underflow,
    )
    // assert!(output.len() >= prefix_len);
}

#[cfg(test)]
mod test {
    use super::the_trait::Conv;
    use super::*;
    fn s(q: &str) -> Result<String, ()> {
        Ok(q.to_owned())
    }
    #[test]
    fn broken_paths() {
        assert!(to_canon_path(r"\\").is_err());
        assert!(to_canon_path(r"\\.").is_err());
        assert!(to_canon_path(r"\\\abc").is_err());
    }
    #[test]
    fn remove_dotdot_test() {
        let (fst, snd) = remove_dotdot(&to_u16(r"C:\..\NUL").unwrap(), 3, VERBATIM_PREFIX);
        let fst: String = <&str as Conv>::rev_conv(fst);
        assert_eq!((fst, snd), (s(r"\\?\C:\NUL").unwrap(), false));
    }
    #[test]
    fn unc_path() {
        assert_eq!(
            to_canon_path(r"\\localhost\pipe\alpha\.."),
            s(r"\\?\UNC\localhost\pipe\alpha\..")
        );
    }
    #[test]
    fn device_path_unc() {
        assert_eq!(
            to_canon_path(r"\\.\unc\localhost\pipe\gamma\.."),
            s(r"\\?\UNC\localhost\pipe\gamma\..")
        );
        assert_eq!(
            to_canon_path(r"//.\UnC/localhost/pipe/delta/epsilon"),
            s(r"\\?\UNC\localhost\pipe\delta/epsilon")
        );
        assert_eq!(to_canon_path(r"\\.\C:\alpha/beta"), s(r"\\?\C:\alpha\beta"));
        assert_eq!(to_canon_path(r"\\.\C:\.."), s(r"\\?\C:\"));
    }
    #[test]
    fn no_process_legacy_dos_devices_or_extra_dotdot() {
        assert_eq!(to_canon_path(r"C:\..\NUL"), s(r"\\?\C:\NUL"))
    }
    #[test]
    fn does_normalize_ordinary_files() {
        assert_eq!(to_canon_path(r"C:/alpha/beta"), s(r"\\?\C:\alpha\beta"));
        assert_eq!(to_canon_path(r"C:\alpha/.."), s(r"\\?\C:\"));
        assert_eq!(to_canon_path(r"C:\/.."), s(r"\\?\C:\"))
    }
    #[test]
    fn respects_verbatim_paths() {
        assert_eq!(to_canon_path(r"\\?\C:/alpha/beta"), s(r"\\?\C:/alpha/beta"));
        assert_eq!(to_canon_path(r"\??\C:/alpha/beta"), s(r"\\?\C:/alpha/beta"))
    }
    #[test]
    fn accepts_drive_paths() {
        assert_eq!(to_canon_path(r"\\.\C:"), s(r"\\?\C:"))
    }
    #[test]
    fn slashes_pipe_paths() {
        assert_eq!(
            to_canon_path(r"\\.\pipe//alpha/beta"),
            s(r"\\?\pipe\/alpha/beta")
        )
    }
    #[test]
    fn drive_relative_path() {
        assert_eq!(to_canon_path(r"\a/b/.."), s(r"\\?\C:\a"));
    }
    #[test]
    fn relative_path() {
        assert_eq!(to_canon_path(r"a/b"), s(r"\\?\C:\alpha\a\b"));
    }
    #[test]
    fn odd_path() {
        assert_eq!(to_canon_path(r"C:a/b"), s(r"\\?\C:\alpha\a\b"));
    }
}
