
// mod ::core::QByteArray
// package qtcore
// /usr/include/qt/QtCore/qbytearray.h
// #include <qbytearray.h>
// #include <QtCore>

//  header block end

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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QByteArrayList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QByteArrayList {
pub fn Operator_equal_0(&self) -> usize {
    // QByteArrayList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QByteArrayList {
pub fn Operator_equal_1(&self) -> usize {
    // QByteArrayList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QByteArrayList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QByteArrayList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QByteArrayList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QByteArrayList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QByteArrayList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QByteArrayList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QByteArrayList {
pub fn Size_0(&self) -> i32 {
    // QByteArrayList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QByteArrayList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QByteArrayList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QByteArrayList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QByteArrayList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QByteArrayList {
pub fn IsDetached_0(&self) -> bool {
    // QByteArrayList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QByteArrayList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QByteArrayList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QByteArrayList {
pub fn IsSharedWith_0(&self) -> bool {
    // QByteArrayList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QByteArrayList {
pub fn IsEmpty_0(&self) -> bool {
    // QByteArrayList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QByteArrayList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QByteArrayList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QByteArrayList {
pub fn At_0(&self) -> usize {
    // QByteArrayList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QByteArrayList {
pub fn Operator_get_index_0(&self) -> usize {
    // QByteArrayList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QByteArrayList {
pub fn Operator_get_index_1(&self) -> usize {
    // QByteArrayList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QByteArrayList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QByteArrayList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QByteArrayList {
pub fn Append_0(&self) -> (/*void*/) {
    // QByteArrayList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QByteArrayList {
pub fn Append_1(&self) -> (/*void*/) {
    // QByteArrayList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QByteArrayList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QByteArrayList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QByteArrayList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QByteArrayList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QByteArrayList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QByteArrayList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QByteArrayList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QByteArrayList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QByteArrayList {
pub fn RemoveAll_0(&self) -> i32 {
    // QByteArrayList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QByteArrayList {
pub fn RemoveOne_0(&self) -> bool {
    // QByteArrayList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QByteArrayList {
pub fn TakeAt_0(&self) -> usize {
    // QByteArrayList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QByteArrayList {
pub fn TakeFirst_0(&self) -> usize {
    // QByteArrayList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QByteArrayList {
pub fn TakeLast_0(&self) -> usize {
    // QByteArrayList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QByteArrayList {
pub fn Move_0(&self) -> (/*void*/) {
    // QByteArrayList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QByteArrayList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QByteArrayList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QByteArrayList {
pub fn IndexOf_0(&self) -> i32 {
    // QByteArrayList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QByteArrayList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QByteArrayList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QByteArrayList {
pub fn Contains_0(&self) -> bool {
    // QByteArrayList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QByteArrayList {
pub fn Count_0(&self) -> i32 {
    // QByteArrayList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QByteArrayList {
pub fn Begin_0(&self) -> usize {
    // QByteArrayList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QByteArrayList {
pub fn Begin_1(&self) -> usize {
    // QByteArrayList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QByteArrayList {
pub fn Cbegin_0(&self) -> usize {
    // QByteArrayList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QByteArrayList {
pub fn ConstBegin_0(&self) -> usize {
    // QByteArrayList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QByteArrayList {
pub fn End_0(&self) -> usize {
    // QByteArrayList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QByteArrayList {
pub fn End_1(&self) -> usize {
    // QByteArrayList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QByteArrayList {
pub fn Cend_0(&self) -> usize {
    // QByteArrayList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QByteArrayList {
pub fn ConstEnd_0(&self) -> usize {
    // QByteArrayList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QByteArrayList {
pub fn Rbegin_0(&self) -> usize {
    // QByteArrayList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QByteArrayList {
pub fn Rend_0(&self) -> usize {
    // QByteArrayList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QByteArrayList {
pub fn Rbegin_1(&self) -> usize {
    // QByteArrayList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QByteArrayList {
pub fn Rend_1(&self) -> usize {
    // QByteArrayList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QByteArrayList {
pub fn Crbegin_0(&self) -> usize {
    // QByteArrayList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QByteArrayList {
pub fn Crend_0(&self) -> usize {
    // QByteArrayList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QByteArrayList {
pub fn Insert_1(&self) -> usize {
    // QByteArrayList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QByteArrayList {
pub fn Erase_0(&self) -> usize {
    // QByteArrayList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QByteArrayList {
pub fn Erase_1(&self) -> usize {
    // QByteArrayList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QByteArrayList {
pub fn Count_1(&self) -> i32 {
    // QByteArrayList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QByteArrayList {
pub fn Length_0(&self) -> i32 {
    // QByteArrayList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QByteArrayList {
pub fn First_0(&self) -> usize {
    // QByteArrayList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QByteArrayList {
pub fn ConstFirst_0(&self) -> usize {
    // QByteArrayList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QByteArrayList {
pub fn First_1(&self) -> usize {
    // QByteArrayList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QByteArrayList {
pub fn Last_0(&self) -> usize {
    // QByteArrayList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QByteArrayList {
pub fn Last_1(&self) -> usize {
    // QByteArrayList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QByteArrayList {
pub fn ConstLast_0(&self) -> usize {
    // QByteArrayList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QByteArrayList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QByteArrayList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QByteArrayList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QByteArrayList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QByteArrayList {
pub fn StartsWith_0(&self) -> bool {
    // QByteArrayList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QByteArrayList {
pub fn EndsWith_0(&self) -> bool {
    // QByteArrayList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QByteArrayList {
pub fn Mid_0(&self) -> usize {
    // QByteArrayList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QByteArrayList {
pub fn Value_0(&self) -> usize {
    // QByteArrayList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QByteArrayList {
pub fn Value_1(&self) -> usize {
    // QByteArrayList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QByteArrayList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QByteArrayList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QByteArrayList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QByteArrayList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QByteArrayList {
pub fn Front_0(&self) -> usize {
    // QByteArrayList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QByteArrayList {
pub fn Front_1(&self) -> usize {
    // QByteArrayList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QByteArrayList {
pub fn Back_0(&self) -> usize {
    // QByteArrayList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QByteArrayList {
pub fn Back_1(&self) -> usize {
    // QByteArrayList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QByteArrayList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QByteArrayList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QByteArrayList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QByteArrayList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QByteArrayList {
pub fn Empty_0(&self) -> bool {
    // QByteArrayList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QByteArrayList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QByteArrayList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QByteArrayList {
pub fn Operator_add_0(&self) -> usize {
    // QByteArrayList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QByteArrayList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QByteArrayList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QByteArrayList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QByteArrayList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QByteArrayList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QByteArrayList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QByteArrayList {
pub fn ToVector_0(&self) -> usize {
    // QByteArrayList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QByteArrayList {
pub fn ToSet_0(&self) -> usize {
    // QByteArrayList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QByteArrayList {
pub fn FromVector_0(&self) -> usize {
    // QByteArrayList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QByteArrayList {
pub fn FromSet_0(&self) -> usize {
    // QByteArrayList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QByteArrayList {
pub fn FromStdList_0(&self) -> usize {
    // QByteArrayList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QByteArrayList {
pub fn ToStdList_0(&self) -> i32 {
    // QByteArrayList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QByteArrayList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QByteArrayList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QByteArrayList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QByteArrayList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QByteArrayList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QByteArrayList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QByteArrayList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QByteArrayList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QByteArrayList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QByteArrayList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QByteArrayList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QByteArrayList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QByteArrayList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QByteArrayList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QByteArrayList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QByteArrayList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QByteArrayList {
pub fn IsValidIterator_0(&self) -> bool {
    // QByteArrayList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QByteArrayList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QByteArrayList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QByteArrayList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QByteArrayList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QByteArrayList {
pub fn Contains_impl_0(&self) -> bool {
    // QByteArrayList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QByteArrayList {
pub fn Contains_impl_1(&self) -> bool {
    // QByteArrayList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QByteArrayList {
pub fn Count_impl_0(&self) -> i32 {
    // QByteArrayList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QByteArrayList {
pub fn Count_impl_1(&self) -> i32 {
    // QByteArrayList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QByteArrayList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
