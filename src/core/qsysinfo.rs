

// mod ::core::QSysInfo
// package qtcore
// /usr/include/qt/QtCore/qsysinfo.h
// #include <qsysinfo.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QSysInfo)=1
pub struct QSysInfo {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSysInfo_ITF interface {
//    QSysInfo_PTR() *QSysInfo
//}
//func (ptr *QSysInfo) QSysInfo_PTR() *QSysInfo { return ptr }

impl /*struct*/ QSysInfo {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSysInfo {
    return QSysInfo{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSysInfo {
//  type Target = QSysInfoBASE;
//
//  fn deref(&self) -> &QSysInfoBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSysInfoBASE> for QSysInfo {
//  fn as_ref(& self) -> & QSysInfoBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsysinfo.h:220
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QSysInfo::WinVersion windowsVersion()

/*

*/
impl /*struct*/ QSysInfo {
  pub fn windowsVersion_0<RetType, T: QSysInfo_windowsVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowsVersion_0();
    // return 1;
  }
}
pub trait QSysInfo_windowsVersion_0<RetType> {
  fn windowsVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_windowsVersion_0<i32> for () {
  fn windowsVersion_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo14windowsVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:227
// index:0
// Public static inline Visibility=Default Availability=Available
// [4] QSysInfo::MacVersion macVersion()

/*

*/
impl /*struct*/ QSysInfo {
  pub fn macVersion_0<RetType, T: QSysInfo_macVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.macVersion_0();
    // return 1;
  }
}
pub trait QSysInfo_macVersion_0<RetType> {
  fn macVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_macVersion_0<i32> for () {
  fn macVersion_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo10macVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:232
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString buildCpuArchitecture()

/*
Returns the architecture of the CPU that Qt was compiled for, in text format. Note that this may not match the actual CPU that the application is running on if there's an emulation layer or if the CPU supports multiple architectures (like x86-64 processors supporting i386 applications). To detect that, use currentCpuArchitecture().

Values returned by this function are stable and will not change over time, so applications can rely on the returned value as an identifier, except that new CPU types may be added over time.

Typical returned values are (note: list not exhaustive):


"arm"
"arm64"
"i386"
"ia64"
"mips"
"mips64"
"power"
"power64"
"sparc"
"sparcv9"
"x86_64"


This function was introduced in  Qt 5.4.

See also QSysInfo::buildAbi() and QSysInfo::currentCpuArchitecture().
*/
impl /*struct*/ QSysInfo {
  pub fn buildCpuArchitecture_0<RetType, T: QSysInfo_buildCpuArchitecture_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.buildCpuArchitecture_0();
    // return 1;
  }
}
pub trait QSysInfo_buildCpuArchitecture_0<RetType> {
  fn buildCpuArchitecture_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_buildCpuArchitecture_0<usize> for () {
  fn buildCpuArchitecture_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo20buildCpuArchitectureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:233
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString currentCpuArchitecture()

/*
Returns the architecture of the CPU that the application is running on, in text format. Note that this function depends on what the OS will report and may not detect the actual CPU architecture if the OS hides that information or is unable to provide it. For example, a 32-bit OS running on a 64-bit CPU is usually unable to determine the CPU is actually capable of running 64-bit programs.

Values returned by this function are mostly stable: an attempt will be made to ensure that they stay constant over time and match the values returned by QSysInfo::builldCpuArchitecture(). However, due to the nature of the operating system functions being used, there may be discrepancies.

Typical returned values are (note: list not exhaustive):


"arm"
"arm64"
"i386"
"ia64"
"mips"
"mips64"
"power"
"power64"
"sparc"
"sparcv9"
"x86_64"


This function was introduced in  Qt 5.4.

See also QSysInfo::buildAbi() and QSysInfo::buildCpuArchitecture().
*/
impl /*struct*/ QSysInfo {
  pub fn currentCpuArchitecture_0<RetType, T: QSysInfo_currentCpuArchitecture_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentCpuArchitecture_0();
    // return 1;
  }
}
pub trait QSysInfo_currentCpuArchitecture_0<RetType> {
  fn currentCpuArchitecture_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_currentCpuArchitecture_0<usize> for () {
  fn currentCpuArchitecture_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo22currentCpuArchitectureEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:234
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString buildAbi()

/*
Returns the full architecture string that Qt was compiled for. This string is useful for identifying different, incompatible builds. For example, it can be used as an identifier to request an upgrade package from a server.

The values returned from this function are kept stable as follows: the mandatory components of the result will not change in future versions of Qt, but optional suffixes may be added.

The returned value is composed of three or more parts, separated by dashes ("-"). They are:


 ComponentValue
CPU ArchitectureThe same as QSysInfo::buildCpuArchitecture(), such as "arm", "i386", "mips" or "x86_64"
Endianness"little_endian" or "big_endian"
Word sizeWhether it's a 32- or 64-bit application. Possible values are: "llp64" (Windows 64-bit), "lp64" (Unix 64-bit), "ilp32" (32-bit)
(Optional) ABIZero or more components identifying different ABIs possible in this architecture. Currently, Qt has optional ABI components for ARM and MIPS processors: one component is the main ABI (such as "eabi", "o32", "n32", "o64"); another is whether the calling convention is using hardware floating point registers ("hardfloat" is present).Additionally, if Qt was configured with -qreal float, the ABI option tag "qreal_float" will be present. If Qt was configured with another type as qreal, that type is present after "qreal_", with all characters other than letters and digits escaped by an underscore, followed by two hex digits. For example, -qreal long double becomes "qreal_long_20double".



This function was introduced in  Qt 5.4.

See also QSysInfo::buildCpuArchitecture().
*/
impl /*struct*/ QSysInfo {
  pub fn buildAbi_0<RetType, T: QSysInfo_buildAbi_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.buildAbi_0();
    // return 1;
  }
}
pub trait QSysInfo_buildAbi_0<RetType> {
  fn buildAbi_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_buildAbi_0<usize> for () {
  fn buildAbi_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo8buildAbiEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:236
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString kernelType()

/*
Returns the type of the operating system kernel Qt was compiled for. It's also the kernel the application is running on, unless the host operating system is running a form of compatibility or virtualization layer.

Values returned by this function are stable and will not change over time, so applications can rely on the returned value as an identifier, except that new OS kernel types may be added over time.

On Windows, this function returns the type of Windows kernel, like "winnt". On Unix systems, it returns the same as the output of uname -s (lowercased).

Note: This function may return surprising values: it returns "linux" for all operating systems running Linux (including Android), "qnx" for all operating systems running QNX, "freebsd" for Debian/kFreeBSD, and "darwin" for macOS and iOS. For information on the type of product the application is running on, see productType().

This function was introduced in  Qt 5.4.

See also QFileSelector, kernelVersion(), productType(), productVersion(), and prettyProductName().
*/
impl /*struct*/ QSysInfo {
  pub fn kernelType_0<RetType, T: QSysInfo_kernelType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.kernelType_0();
    // return 1;
  }
}
pub trait QSysInfo_kernelType_0<RetType> {
  fn kernelType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_kernelType_0<usize> for () {
  fn kernelType_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo10kernelTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:237
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString kernelVersion()

/*
Returns the release version of the operating system kernel. On Windows, it returns the version of the NT kernel. On Unix systems, including Android and macOS, it returns the same as the uname -r command would return.

If the version could not be determined, this function may return an empty string.

This function was introduced in  Qt 5.4.

See also kernelType(), productType(), productVersion(), and prettyProductName().
*/
impl /*struct*/ QSysInfo {
  pub fn kernelVersion_0<RetType, T: QSysInfo_kernelVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.kernelVersion_0();
    // return 1;
  }
}
pub trait QSysInfo_kernelVersion_0<RetType> {
  fn kernelVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_kernelVersion_0<usize> for () {
  fn kernelVersion_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo13kernelVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:238
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString productType()

/*
Returns the product name of the operating system this application is running in. If the application is running on some sort of emulation or virtualization layer (such as WINE on a Unix system), this function will inspect the emulation / virtualization layer.

Values returned by this function are stable and will not change over time, so applications can rely on the returned value as an identifier, except that new OS types may be added over time.

Linux and Android note: this function returns "android" for Linux systems running Android userspace, notably when using the Bionic library. For all other Linux systems, regardless of C library being used, it tries to determine the distribution name and returns that. If determining the distribution name failed, it returns "unknown".

macOS note: this function returns "osx" for all macOS systems, regardless of Apple naming convention. The returned string will be updated for Qt 6. Note that this function erroneously returned "macos" for macOS 10.12 in Qt versions 5.6.2, 5.7.1, and 5.8.0.

Darwin, iOS, tvOS, and watchOS note: this function returns "ios" for iOS systems, "tvos" for tvOS systems, "watchos" for watchOS systems, and "darwin" in case the system could not be determined.

FreeBSD note: this function returns "debian" for Debian/kFreeBSD and "unknown" otherwise.

Windows note: this function "winrt" for WinRT builds, and "windows" for normal desktop builds.

For other Unix-type systems, this function usually returns "unknown".

This function was introduced in  Qt 5.4.

See also QFileSelector, kernelType(), kernelVersion(), productVersion(), and prettyProductName().
*/
impl /*struct*/ QSysInfo {
  pub fn productType_0<RetType, T: QSysInfo_productType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.productType_0();
    // return 1;
  }
}
pub trait QSysInfo_productType_0<RetType> {
  fn productType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_productType_0<usize> for () {
  fn productType_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo11productTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:239
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString productVersion()

/*
Returns the product version of the operating system in string form. If the version could not be determined, this function returns "unknown".

It will return the Android, iOS, macOS, Windows full-product versions on those systems.

Typical returned values are (note: list not exhaustive):


"2016.09" (Amazon Linux AMI 2016.09)
"7.1" (Android Nougat)
"25" (Fedora 25)
"10.1" (iOS 10.1)
"10.12" (macOS Sierra)
"10.0" (tvOS 10)
"16.10" (Ubuntu 16.10)
"3.1" (watchOS 3.1)
"7 SP 1" (Windows 7 Service Pack 1)
"8.1" (Windows 8.1)
"10" (Windows 10)
"Server 2016" (Windows Server 2016)


On Linux systems, it will try to determine the distribution version and will return that. This is also done on Debian/kFreeBSD, so this function will return Debian version in that case.

In all other Unix-type systems, this function always returns "unknown".

Note: The version string returned from this function is not guaranteed to be orderable. On Linux, the version of the distribution may jump unexpectedly, please refer to the distribution's documentation for versioning practices.

This function was introduced in  Qt 5.4.

See also kernelType(), kernelVersion(), productType(), and prettyProductName().
*/
impl /*struct*/ QSysInfo {
  pub fn productVersion_0<RetType, T: QSysInfo_productVersion_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.productVersion_0();
    // return 1;
  }
}
pub trait QSysInfo_productVersion_0<RetType> {
  fn productVersion_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_productVersion_0<usize> for () {
  fn productVersion_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo14productVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:240
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString prettyProductName()

/*
Returns a prettier form of productType() and productVersion(), containing other tokens like the operating system type, codenames and other information. The result of this function is suitable for displaying to the user, but not for long-term storage, as the string may change with updates to Qt.

If productType() is "unknown", this function will instead use the kernelType() and kernelVersion() functions.

This function was introduced in  Qt 5.4.

See also kernelType(), kernelVersion(), productType(), and productVersion().
*/
impl /*struct*/ QSysInfo {
  pub fn prettyProductName_0<RetType, T: QSysInfo_prettyProductName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.prettyProductName_0();
    // return 1;
  }
}
pub trait QSysInfo_prettyProductName_0<RetType> {
  fn prettyProductName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_prettyProductName_0<usize> for () {
  fn prettyProductName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo17prettyProductNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsysinfo.h:242
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString machineHostName()

/*
Returns this machine's host name, if one is configured. Note that hostnames are not guaranteed to be globally unique, especially if they were configured automatically.

This function does not guarantee the returned host name is a Fully Qualified Domain Name (FQDN). For that, use QHostInfo to resolve the returned name to an FQDN.

This function returns the same as QHostInfo::localHostName().

This function was introduced in  Qt 5.6.

See also QHostInfo::localDomainName.
*/
impl /*struct*/ QSysInfo {
  pub fn machineHostName_0<RetType, T: QSysInfo_machineHostName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.machineHostName_0();
    // return 1;
  }
}
pub trait QSysInfo_machineHostName_0<RetType> {
  fn machineHostName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSysInfo_machineHostName_0<usize> for () {
  fn machineHostName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QSysInfo15machineHostNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQSysInfo(this :*mut QSysInfo) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN8QSysInfoD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum provides platform-specific information about the sizes of data structures used by the underlying architecture.


*/
pub type QSysInfo__Sizes = i32;
// 
pub const QSysInfo__WordSize :QSysInfo__Sizes = 64;
pub fn QSysInfo_SizesItemName(val: i32) ->String {
  match val {
     QSysInfo__WordSize => // 64
     {return String::from("WordSize");}
  _ => {return format!("{}", val);}
}
}
pub fn QSysInfo_SizesItemName_s(val: i32) ->String {
  //var nilthis *QSysInfo
  //return nilthis.SizesItemName(val);
  return QSysInfo_SizesItemName(val);
}


/*
QSysInfo::ByteOrderBigEndian or LittleEndianEquals BigEndian or LittleEndian, depending on the platform's byte order.

*/
pub type QSysInfo__Endian = i32;
// Big-endian byte order (also called Network byte order)
pub const QSysInfo__BigEndian :QSysInfo__Endian = 0;
// Little-endian byte order
pub const QSysInfo__LittleEndian :QSysInfo__Endian = 1;
// 
pub const QSysInfo__ByteOrder :QSysInfo__Endian = 1;
pub fn QSysInfo_EndianItemName(val: i32) ->String {
  match val {
     QSysInfo__BigEndian => // 0
     {return String::from("BigEndian");}
     QSysInfo__LittleEndian => // 1
     {return String::from("LittleEndian,ByteOrder");}
    // QSysInfo__ByteOrder => // 1
    // {return String::from("");}
  _ => {return format!("{}", val);}
}
}
pub fn QSysInfo_EndianItemName_s(val: i32) ->String {
  //var nilthis *QSysInfo
  //return nilthis.EndianItemName(val);
  return QSysInfo_EndianItemName(val);
}


/*


*/
pub type QSysInfo__WinVersion = i32;
// 
pub const QSysInfo__WV_None :QSysInfo__WinVersion = 0;
// 
pub const QSysInfo__WV_32s :QSysInfo__WinVersion = 1;
// 
pub const QSysInfo__WV_95 :QSysInfo__WinVersion = 2;
// 
pub const QSysInfo__WV_98 :QSysInfo__WinVersion = 3;
// 
pub const QSysInfo__WV_Me :QSysInfo__WinVersion = 4;
// 
pub const QSysInfo__WV_DOS_based :QSysInfo__WinVersion = 15;
// 
pub const QSysInfo__WV_NT :QSysInfo__WinVersion = 16;
// 
pub const QSysInfo__WV_2000 :QSysInfo__WinVersion = 32;
// 
pub const QSysInfo__WV_XP :QSysInfo__WinVersion = 48;
// 
pub const QSysInfo__WV_2003 :QSysInfo__WinVersion = 64;
// 
pub const QSysInfo__WV_VISTA :QSysInfo__WinVersion = 128;
// 
pub const QSysInfo__WV_WINDOWS7 :QSysInfo__WinVersion = 144;
// 
pub const QSysInfo__WV_WINDOWS8 :QSysInfo__WinVersion = 160;
// 
pub const QSysInfo__WV_WINDOWS8_1 :QSysInfo__WinVersion = 176;
// 
pub const QSysInfo__WV_WINDOWS10 :QSysInfo__WinVersion = 192;
// 
pub const QSysInfo__WV_NT_based :QSysInfo__WinVersion = 240;
// 
pub const QSysInfo__WV_4_0 :QSysInfo__WinVersion = 16;
// 
pub const QSysInfo__WV_5_0 :QSysInfo__WinVersion = 32;
// 
pub const QSysInfo__WV_5_1 :QSysInfo__WinVersion = 48;
// 
pub const QSysInfo__WV_5_2 :QSysInfo__WinVersion = 64;
// 
pub const QSysInfo__WV_6_0 :QSysInfo__WinVersion = 128;
// 
pub const QSysInfo__WV_6_1 :QSysInfo__WinVersion = 144;
// 
pub const QSysInfo__WV_6_2 :QSysInfo__WinVersion = 160;
// 
pub const QSysInfo__WV_6_3 :QSysInfo__WinVersion = 176;
// 
pub const QSysInfo__WV_10_0 :QSysInfo__WinVersion = 192;
// 
pub const QSysInfo__WV_CE :QSysInfo__WinVersion = 256;
// 
pub const QSysInfo__WV_CENET :QSysInfo__WinVersion = 512;
// 
pub const QSysInfo__WV_CE_5 :QSysInfo__WinVersion = 768;
// 
pub const QSysInfo__WV_CE_6 :QSysInfo__WinVersion = 1024;
// 
pub const QSysInfo__WV_CE_based :QSysInfo__WinVersion = 3840;
pub fn QSysInfo_WinVersionItemName(val: i32) ->String {
  match val {
     QSysInfo__WV_None => // 0
     {return String::from("WV_None");}
     QSysInfo__WV_32s => // 1
     {return String::from("WV_32s");}
     QSysInfo__WV_95 => // 2
     {return String::from("WV_95");}
     QSysInfo__WV_98 => // 3
     {return String::from("WV_98");}
     QSysInfo__WV_Me => // 4
     {return String::from("WV_Me");}
     QSysInfo__WV_DOS_based => // 15
     {return String::from("WV_DOS_based");}
     QSysInfo__WV_NT => // 16
     {return String::from("WV_NT,WV_4_0");}
     QSysInfo__WV_2000 => // 32
     {return String::from("WV_2000,WV_5_0");}
     QSysInfo__WV_XP => // 48
     {return String::from("WV_XP,WV_5_1");}
     QSysInfo__WV_2003 => // 64
     {return String::from("WV_2003,WV_5_2");}
     QSysInfo__WV_VISTA => // 128
     {return String::from("WV_VISTA,WV_6_0");}
     QSysInfo__WV_WINDOWS7 => // 144
     {return String::from("WV_WINDOWS7,WV_6_1");}
     QSysInfo__WV_WINDOWS8 => // 160
     {return String::from("WV_WINDOWS8,WV_6_2");}
     QSysInfo__WV_WINDOWS8_1 => // 176
     {return String::from("WV_WINDOWS8_1,WV_6_3");}
     QSysInfo__WV_WINDOWS10 => // 192
     {return String::from("WV_WINDOWS10,WV_10_0");}
     QSysInfo__WV_NT_based => // 240
     {return String::from("WV_NT_based");}
    // QSysInfo__WV_4_0 => // 16
    // {return String::from("");}
    // QSysInfo__WV_5_0 => // 32
    // {return String::from("");}
    // QSysInfo__WV_5_1 => // 48
    // {return String::from("");}
    // QSysInfo__WV_5_2 => // 64
    // {return String::from("");}
    // QSysInfo__WV_6_0 => // 128
    // {return String::from("");}
    // QSysInfo__WV_6_1 => // 144
    // {return String::from("");}
    // QSysInfo__WV_6_2 => // 160
    // {return String::from("");}
    // QSysInfo__WV_6_3 => // 176
    // {return String::from("");}
    // QSysInfo__WV_10_0 => // 192
    // {return String::from("");}
     QSysInfo__WV_CE => // 256
     {return String::from("WV_CE");}
     QSysInfo__WV_CENET => // 512
     {return String::from("WV_CENET");}
     QSysInfo__WV_CE_5 => // 768
     {return String::from("WV_CE_5");}
     QSysInfo__WV_CE_6 => // 1024
     {return String::from("WV_CE_6");}
     QSysInfo__WV_CE_based => // 3840
     {return String::from("WV_CE_based");}
  _ => {return format!("{}", val);}
}
}
pub fn QSysInfo_WinVersionItemName_s(val: i32) ->String {
  //var nilthis *QSysInfo
  //return nilthis.WinVersionItemName(val);
  return QSysInfo_WinVersionItemName(val);
}


/*


*/
pub type QSysInfo__MacVersion = i32;
// 
pub const QSysInfo__MV_None :QSysInfo__MacVersion = 65535;
// 
pub const QSysInfo__MV_Unknown :QSysInfo__MacVersion = 0;
// 
pub const QSysInfo__MV_9 :QSysInfo__MacVersion = 1;
// 
pub const QSysInfo__MV_10_0 :QSysInfo__MacVersion = 2;
// 
pub const QSysInfo__MV_10_1 :QSysInfo__MacVersion = 3;
// 
pub const QSysInfo__MV_10_2 :QSysInfo__MacVersion = 4;
// 
pub const QSysInfo__MV_10_3 :QSysInfo__MacVersion = 5;
// 
pub const QSysInfo__MV_10_4 :QSysInfo__MacVersion = 6;
// 
pub const QSysInfo__MV_10_5 :QSysInfo__MacVersion = 7;
// 
pub const QSysInfo__MV_10_6 :QSysInfo__MacVersion = 8;
// 
pub const QSysInfo__MV_10_7 :QSysInfo__MacVersion = 9;
// 
pub const QSysInfo__MV_10_8 :QSysInfo__MacVersion = 10;
// 
pub const QSysInfo__MV_10_9 :QSysInfo__MacVersion = 11;
// 
pub const QSysInfo__MV_10_10 :QSysInfo__MacVersion = 12;
// 
pub const QSysInfo__MV_10_11 :QSysInfo__MacVersion = 13;
// 
pub const QSysInfo__MV_10_12 :QSysInfo__MacVersion = 14;
// 
pub const QSysInfo__MV_CHEETAH :QSysInfo__MacVersion = 2;
// 
pub const QSysInfo__MV_PUMA :QSysInfo__MacVersion = 3;
// 
pub const QSysInfo__MV_JAGUAR :QSysInfo__MacVersion = 4;
// 
pub const QSysInfo__MV_PANTHER :QSysInfo__MacVersion = 5;
// 
pub const QSysInfo__MV_TIGER :QSysInfo__MacVersion = 6;
// 
pub const QSysInfo__MV_LEOPARD :QSysInfo__MacVersion = 7;
// 
pub const QSysInfo__MV_SNOWLEOPARD :QSysInfo__MacVersion = 8;
// 
pub const QSysInfo__MV_LION :QSysInfo__MacVersion = 9;
// 
pub const QSysInfo__MV_MOUNTAINLION :QSysInfo__MacVersion = 10;
// 
pub const QSysInfo__MV_MAVERICKS :QSysInfo__MacVersion = 11;
// 
pub const QSysInfo__MV_YOSEMITE :QSysInfo__MacVersion = 12;
// 
pub const QSysInfo__MV_ELCAPITAN :QSysInfo__MacVersion = 13;
// 
pub const QSysInfo__MV_SIERRA :QSysInfo__MacVersion = 14;
// 
pub const QSysInfo__MV_IOS :QSysInfo__MacVersion = 256;
// 
pub const QSysInfo__MV_IOS_4_3 :QSysInfo__MacVersion = 323;
// 
pub const QSysInfo__MV_IOS_5_0 :QSysInfo__MacVersion = 336;
// 
pub const QSysInfo__MV_IOS_5_1 :QSysInfo__MacVersion = 337;
// 
pub const QSysInfo__MV_IOS_6_0 :QSysInfo__MacVersion = 352;
// 
pub const QSysInfo__MV_IOS_6_1 :QSysInfo__MacVersion = 353;
// 
pub const QSysInfo__MV_IOS_7_0 :QSysInfo__MacVersion = 368;
// 
pub const QSysInfo__MV_IOS_7_1 :QSysInfo__MacVersion = 369;
// 
pub const QSysInfo__MV_IOS_8_0 :QSysInfo__MacVersion = 384;
// 
pub const QSysInfo__MV_IOS_8_1 :QSysInfo__MacVersion = 385;
// 
pub const QSysInfo__MV_IOS_8_2 :QSysInfo__MacVersion = 386;
// 
pub const QSysInfo__MV_IOS_8_3 :QSysInfo__MacVersion = 387;
// 
pub const QSysInfo__MV_IOS_8_4 :QSysInfo__MacVersion = 388;
// 
pub const QSysInfo__MV_IOS_9_0 :QSysInfo__MacVersion = 400;
// 
pub const QSysInfo__MV_IOS_9_1 :QSysInfo__MacVersion = 401;
// 
pub const QSysInfo__MV_IOS_9_2 :QSysInfo__MacVersion = 402;
// 
pub const QSysInfo__MV_IOS_9_3 :QSysInfo__MacVersion = 403;
// 
pub const QSysInfo__MV_IOS_10_0 :QSysInfo__MacVersion = 416;
// 
pub const QSysInfo__MV_TVOS :QSysInfo__MacVersion = 512;
// 
pub const QSysInfo__MV_TVOS_9_0 :QSysInfo__MacVersion = 656;
// 
pub const QSysInfo__MV_TVOS_9_1 :QSysInfo__MacVersion = 657;
// 
pub const QSysInfo__MV_TVOS_9_2 :QSysInfo__MacVersion = 658;
// 
pub const QSysInfo__MV_TVOS_10_0 :QSysInfo__MacVersion = 672;
// 
pub const QSysInfo__MV_WATCHOS :QSysInfo__MacVersion = 1024;
// 
pub const QSysInfo__MV_WATCHOS_2_0 :QSysInfo__MacVersion = 1056;
// 
pub const QSysInfo__MV_WATCHOS_2_1 :QSysInfo__MacVersion = 1057;
// 
pub const QSysInfo__MV_WATCHOS_2_2 :QSysInfo__MacVersion = 1058;
// 
pub const QSysInfo__MV_WATCHOS_3_0 :QSysInfo__MacVersion = 1072;
pub fn QSysInfo_MacVersionItemName(val: i32) ->String {
  match val {
     QSysInfo__MV_None => // 65535
     {return String::from("MV_None");}
     QSysInfo__MV_Unknown => // 0
     {return String::from("MV_Unknown");}
     QSysInfo__MV_9 => // 1
     {return String::from("MV_9");}
     QSysInfo__MV_10_0 => // 2
     {return String::from("MV_10_0,MV_CHEETAH");}
     QSysInfo__MV_10_1 => // 3
     {return String::from("MV_10_1,MV_PUMA");}
     QSysInfo__MV_10_2 => // 4
     {return String::from("MV_10_2,MV_JAGUAR");}
     QSysInfo__MV_10_3 => // 5
     {return String::from("MV_10_3,MV_PANTHER");}
     QSysInfo__MV_10_4 => // 6
     {return String::from("MV_10_4,MV_TIGER");}
     QSysInfo__MV_10_5 => // 7
     {return String::from("MV_10_5,MV_LEOPARD");}
     QSysInfo__MV_10_6 => // 8
     {return String::from("MV_10_6,MV_SNOWLEOPARD");}
     QSysInfo__MV_10_7 => // 9
     {return String::from("MV_10_7,MV_LION");}
     QSysInfo__MV_10_8 => // 10
     {return String::from("MV_10_8,MV_MOUNTAINLION");}
     QSysInfo__MV_10_9 => // 11
     {return String::from("MV_10_9,MV_MAVERICKS");}
     QSysInfo__MV_10_10 => // 12
     {return String::from("MV_10_10,MV_YOSEMITE");}
     QSysInfo__MV_10_11 => // 13
     {return String::from("MV_10_11,MV_ELCAPITAN");}
     QSysInfo__MV_10_12 => // 14
     {return String::from("MV_10_12,MV_SIERRA");}
    // QSysInfo__MV_CHEETAH => // 2
    // {return String::from("");}
    // QSysInfo__MV_PUMA => // 3
    // {return String::from("");}
    // QSysInfo__MV_JAGUAR => // 4
    // {return String::from("");}
    // QSysInfo__MV_PANTHER => // 5
    // {return String::from("");}
    // QSysInfo__MV_TIGER => // 6
    // {return String::from("");}
    // QSysInfo__MV_LEOPARD => // 7
    // {return String::from("");}
    // QSysInfo__MV_SNOWLEOPARD => // 8
    // {return String::from("");}
    // QSysInfo__MV_LION => // 9
    // {return String::from("");}
    // QSysInfo__MV_MOUNTAINLION => // 10
    // {return String::from("");}
    // QSysInfo__MV_MAVERICKS => // 11
    // {return String::from("");}
    // QSysInfo__MV_YOSEMITE => // 12
    // {return String::from("");}
    // QSysInfo__MV_ELCAPITAN => // 13
    // {return String::from("");}
    // QSysInfo__MV_SIERRA => // 14
    // {return String::from("");}
     QSysInfo__MV_IOS => // 256
     {return String::from("MV_IOS");}
     QSysInfo__MV_IOS_4_3 => // 323
     {return String::from("MV_IOS_4_3");}
     QSysInfo__MV_IOS_5_0 => // 336
     {return String::from("MV_IOS_5_0");}
     QSysInfo__MV_IOS_5_1 => // 337
     {return String::from("MV_IOS_5_1");}
     QSysInfo__MV_IOS_6_0 => // 352
     {return String::from("MV_IOS_6_0");}
     QSysInfo__MV_IOS_6_1 => // 353
     {return String::from("MV_IOS_6_1");}
     QSysInfo__MV_IOS_7_0 => // 368
     {return String::from("MV_IOS_7_0");}
     QSysInfo__MV_IOS_7_1 => // 369
     {return String::from("MV_IOS_7_1");}
     QSysInfo__MV_IOS_8_0 => // 384
     {return String::from("MV_IOS_8_0");}
     QSysInfo__MV_IOS_8_1 => // 385
     {return String::from("MV_IOS_8_1");}
     QSysInfo__MV_IOS_8_2 => // 386
     {return String::from("MV_IOS_8_2");}
     QSysInfo__MV_IOS_8_3 => // 387
     {return String::from("MV_IOS_8_3");}
     QSysInfo__MV_IOS_8_4 => // 388
     {return String::from("MV_IOS_8_4");}
     QSysInfo__MV_IOS_9_0 => // 400
     {return String::from("MV_IOS_9_0");}
     QSysInfo__MV_IOS_9_1 => // 401
     {return String::from("MV_IOS_9_1");}
     QSysInfo__MV_IOS_9_2 => // 402
     {return String::from("MV_IOS_9_2");}
     QSysInfo__MV_IOS_9_3 => // 403
     {return String::from("MV_IOS_9_3");}
     QSysInfo__MV_IOS_10_0 => // 416
     {return String::from("MV_IOS_10_0");}
     QSysInfo__MV_TVOS => // 512
     {return String::from("MV_TVOS");}
     QSysInfo__MV_TVOS_9_0 => // 656
     {return String::from("MV_TVOS_9_0");}
     QSysInfo__MV_TVOS_9_1 => // 657
     {return String::from("MV_TVOS_9_1");}
     QSysInfo__MV_TVOS_9_2 => // 658
     {return String::from("MV_TVOS_9_2");}
     QSysInfo__MV_TVOS_10_0 => // 672
     {return String::from("MV_TVOS_10_0");}
     QSysInfo__MV_WATCHOS => // 1024
     {return String::from("MV_WATCHOS");}
     QSysInfo__MV_WATCHOS_2_0 => // 1056
     {return String::from("MV_WATCHOS_2_0");}
     QSysInfo__MV_WATCHOS_2_1 => // 1057
     {return String::from("MV_WATCHOS_2_1");}
     QSysInfo__MV_WATCHOS_2_2 => // 1058
     {return String::from("MV_WATCHOS_2_2");}
     QSysInfo__MV_WATCHOS_3_0 => // 1072
     {return String::from("MV_WATCHOS_3_0");}
  _ => {return format!("{}", val);}
}
}
pub fn QSysInfo_MacVersionItemName_s(val: i32) ->String {
  //var nilthis *QSysInfo
  //return nilthis.MacVersionItemName(val);
  return QSysInfo_MacVersionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
