#include <stdint.h>
typedef void *HANDLE;
typedef HANDLE *PHANDLE;
typedef unsigned long ULONG_PTR;
typedef uint32_t DWORD, ACCESS_MASK;
typedef uint16_t WORD;


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
  ULONG          Length;
  HANDLE          RootDirectory;
  PUNICODE_STRING ObjectName;
  ULONG           Attributes;
  PVOID           SecurityDescriptor;
  PVOID           SecurityQualityOfService;
} OBJECT_ATTRIBUTES, *POBJECT_ATTRIBUTES;

__stdcall NtCreateFile(
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

union LARGE_INTEGER {
  uint64_t QuadPart,
  struct {
    uint32_t LowPart,
    uint32_t HighPart,
  },
};
