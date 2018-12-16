
// mod ::gui::QWindow
// package qtgui
// /usr/include/qt/QtGui/qwindow.h
// #include <qwindow.h>
// #include <QtGui>

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
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QWindowList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QWindowList {
pub fn Operator_equal_0(&self) -> usize {
    // QWindowList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QWindowList {
pub fn Operator_equal_1(&self) -> usize {
    // QWindowList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QWindowList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QWindowList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QWindowList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QWindowList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QWindowList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QWindowList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QWindowList {
pub fn Size_0(&self) -> i32 {
    // QWindowList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QWindowList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QWindowList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QWindowList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QWindowList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QWindowList {
pub fn IsDetached_0(&self) -> bool {
    // QWindowList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QWindowList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QWindowList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QWindowList {
pub fn IsSharedWith_0(&self) -> bool {
    // QWindowList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QWindowList {
pub fn IsEmpty_0(&self) -> bool {
    // QWindowList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QWindowList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QWindowList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QWindowList {
pub fn At_0(&self) -> usize {
    // QWindowList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QWindowList {
pub fn Operator_get_index_0(&self) -> usize {
    // QWindowList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QWindowList {
pub fn Operator_get_index_1(&self) -> usize {
    // QWindowList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QWindowList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QWindowList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QWindowList {
pub fn Append_0(&self) -> (/*void*/) {
    // QWindowList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QWindowList {
pub fn Append_1(&self) -> (/*void*/) {
    // QWindowList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QWindowList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QWindowList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QWindowList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QWindowList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QWindowList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QWindowList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QWindowList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QWindowList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QWindowList {
pub fn RemoveAll_0(&self) -> i32 {
    // QWindowList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QWindowList {
pub fn RemoveOne_0(&self) -> bool {
    // QWindowList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QWindowList {
pub fn TakeAt_0(&self) -> usize {
    // QWindowList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QWindowList {
pub fn TakeFirst_0(&self) -> usize {
    // QWindowList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QWindowList {
pub fn TakeLast_0(&self) -> usize {
    // QWindowList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QWindowList {
pub fn Move_0(&self) -> (/*void*/) {
    // QWindowList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QWindowList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QWindowList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QWindowList {
pub fn IndexOf_0(&self) -> i32 {
    // QWindowList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QWindowList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QWindowList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QWindowList {
pub fn Contains_0(&self) -> bool {
    // QWindowList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QWindowList {
pub fn Count_0(&self) -> i32 {
    // QWindowList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QWindowList {
pub fn Begin_0(&self) -> usize {
    // QWindowList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QWindowList {
pub fn Begin_1(&self) -> usize {
    // QWindowList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QWindowList {
pub fn Cbegin_0(&self) -> usize {
    // QWindowList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QWindowList {
pub fn ConstBegin_0(&self) -> usize {
    // QWindowList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QWindowList {
pub fn End_0(&self) -> usize {
    // QWindowList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QWindowList {
pub fn End_1(&self) -> usize {
    // QWindowList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QWindowList {
pub fn Cend_0(&self) -> usize {
    // QWindowList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QWindowList {
pub fn ConstEnd_0(&self) -> usize {
    // QWindowList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QWindowList {
pub fn Rbegin_0(&self) -> usize {
    // QWindowList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QWindowList {
pub fn Rend_0(&self) -> usize {
    // QWindowList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QWindowList {
pub fn Rbegin_1(&self) -> usize {
    // QWindowList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QWindowList {
pub fn Rend_1(&self) -> usize {
    // QWindowList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QWindowList {
pub fn Crbegin_0(&self) -> usize {
    // QWindowList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QWindowList {
pub fn Crend_0(&self) -> usize {
    // QWindowList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QWindowList {
pub fn Insert_1(&self) -> usize {
    // QWindowList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QWindowList {
pub fn Erase_0(&self) -> usize {
    // QWindowList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QWindowList {
pub fn Erase_1(&self) -> usize {
    // QWindowList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QWindowList {
pub fn Count_1(&self) -> i32 {
    // QWindowList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QWindowList {
pub fn Length_0(&self) -> i32 {
    // QWindowList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QWindowList {
pub fn First_0(&self) -> usize {
    // QWindowList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QWindowList {
pub fn ConstFirst_0(&self) -> usize {
    // QWindowList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QWindowList {
pub fn First_1(&self) -> usize {
    // QWindowList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QWindowList {
pub fn Last_0(&self) -> usize {
    // QWindowList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QWindowList {
pub fn Last_1(&self) -> usize {
    // QWindowList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QWindowList {
pub fn ConstLast_0(&self) -> usize {
    // QWindowList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QWindowList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QWindowList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QWindowList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QWindowList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QWindowList {
pub fn StartsWith_0(&self) -> bool {
    // QWindowList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QWindowList {
pub fn EndsWith_0(&self) -> bool {
    // QWindowList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QWindowList {
pub fn Mid_0(&self) -> usize {
    // QWindowList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QWindowList {
pub fn Value_0(&self) -> usize {
    // QWindowList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QWindowList {
pub fn Value_1(&self) -> usize {
    // QWindowList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QWindowList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QWindowList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QWindowList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QWindowList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QWindowList {
pub fn Front_0(&self) -> usize {
    // QWindowList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QWindowList {
pub fn Front_1(&self) -> usize {
    // QWindowList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QWindowList {
pub fn Back_0(&self) -> usize {
    // QWindowList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QWindowList {
pub fn Back_1(&self) -> usize {
    // QWindowList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QWindowList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QWindowList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QWindowList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QWindowList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QWindowList {
pub fn Empty_0(&self) -> bool {
    // QWindowList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QWindowList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QWindowList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QWindowList {
pub fn Operator_add_0(&self) -> usize {
    // QWindowList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QWindowList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QWindowList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QWindowList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QWindowList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QWindowList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QWindowList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QWindowList {
pub fn ToVector_0(&self) -> usize {
    // QWindowList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QWindowList {
pub fn ToSet_0(&self) -> usize {
    // QWindowList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QWindowList {
pub fn FromVector_0(&self) -> usize {
    // QWindowList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QWindowList {
pub fn FromSet_0(&self) -> usize {
    // QWindowList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QWindowList {
pub fn FromStdList_0(&self) -> usize {
    // QWindowList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QWindowList {
pub fn ToStdList_0(&self) -> i32 {
    // QWindowList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QWindowList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QWindowList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QWindowList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QWindowList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QWindowList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QWindowList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QWindowList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QWindowList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QWindowList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QWindowList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QWindowList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QWindowList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QWindowList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QWindowList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QWindowList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QWindowList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QWindowList {
pub fn IsValidIterator_0(&self) -> bool {
    // QWindowList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QWindowList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QWindowList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QWindowList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QWindowList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QWindowList {
pub fn Contains_impl_0(&self) -> bool {
    // QWindowList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QWindowList {
pub fn Contains_impl_1(&self) -> bool {
    // QWindowList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QWindowList {
pub fn Count_impl_0(&self) -> i32 {
    // QWindowList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QWindowList {
pub fn Count_impl_1(&self) -> i32 {
    // QWindowList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWindowList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
