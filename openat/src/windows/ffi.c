typedef void *HANDLE, *PVOID, *PSID;
typedef long NTSTATUS;
typedef HANDLE *PHANDLE;
typedef unsigned long ULONG, *ULONG_PTR;
typedef unsigned long DWORD, ACCESS_MASK;
typedef unsigned short WORD, USHORT;
typedef unsigned char BYTE, UCHAR;

typedef WORD SECURITY_DESCRIPTOR_CONTROL;
typedef struct _UNICODE_STRING {
  USHORT Length;
  USHORT MaximumLength;
  USHORT *Buffer;
} UNICODE_STRING, *PUNICODE_STRING;


typedef union _LARGE_INTEGER {
  unsigned long long QuadPart;
  struct {
    unsigned long LowPart;
    unsigned long HighPart;
  };
} LARGE_INTEGER, *PLARGE_INTEGER;

typedef struct _ACL {
  BYTE AclRevision;
  BYTE Sbz1;
  WORD AclSize;
  WORD AceCount;
  WORD Sbz2;
} ACL, *PACL;


typedef struct _SECURITY_DESCRIPTOR {
  UCHAR                       Revision;
  UCHAR                       Sbz1;
  SECURITY_DESCRIPTOR_CONTROL Control;
  PSID                        Owner;
  PSID                        Group;
  PACL                        Sacl;
  PACL                        Dacl;
} SECURITY_DESCRIPTOR, *PISECURITY_DESCRIPTOR;

typedef struct _IO_STATUS_BLOCK {
  union {
    NTSTATUS Status;
    PVOID    Pointer;
  } DUMMYUNIONNAME;
  ULONG_PTR Information;
} IO_STATUS_BLOCK, *PIO_STATUS_BLOCK;

typedef struct _OBJECT_ATTRIBUTES {
  ULONG           Length;
  HANDLE          RootDirectory;
  PUNICODE_STRING ObjectName;
  ULONG           Attributes;
  PVOID           SecurityDescriptor;
  PVOID           SecurityQualityOfService;
} OBJECT_ATTRIBUTES, *POBJECT_ATTRIBUTES;

__stdcall NTSTATUS NtCreateFile(
   PHANDLE            FileHandle,
   ACCESS_MASK        DesiredAccess,
   POBJECT_ATTRIBUTES ObjectAttributes,
   PIO_STATUS_BLOCK   IoStatusBlock,
   PLARGE_INTEGER     AllocationSize,
   ULONG              FileAttributes,
   ULONG              ShareAccess,
   ULONG              CreateDisposition,
   ULONG              CreateOptions,
   PVOID              EaBuffer,
   ULONG              EaLength
);
