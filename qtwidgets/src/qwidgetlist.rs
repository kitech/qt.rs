
// mod ::widgets::QWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qwidget.h
// #include <qwidget.h>
// #include <QtWidgets>

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
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin
pub struct QWidgetList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QWidgetList {
pub fn Operator_equal_0(&self) -> usize {
    // QWidgetList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QWidgetList {
pub fn Operator_equal_1(&self) -> usize {
    // QWidgetList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QWidgetList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QWidgetList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QWidgetList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QWidgetList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QWidgetList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QWidgetList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QWidgetList {
pub fn Size_0(&self) -> i32 {
    // QWidgetList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QWidgetList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QWidgetList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QWidgetList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QWidgetList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QWidgetList {
pub fn IsDetached_0(&self) -> bool {
    // QWidgetList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QWidgetList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QWidgetList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QWidgetList {
pub fn IsSharedWith_0(&self) -> bool {
    // QWidgetList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QWidgetList {
pub fn IsEmpty_0(&self) -> bool {
    // QWidgetList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QWidgetList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QWidgetList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QWidgetList {
pub fn At_0(&self) -> usize {
    // QWidgetList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QWidgetList {
pub fn Operator_get_index_0(&self) -> usize {
    // QWidgetList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QWidgetList {
pub fn Operator_get_index_1(&self) -> usize {
    // QWidgetList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QWidgetList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QWidgetList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QWidgetList {
pub fn Append_0(&self) -> (/*void*/) {
    // QWidgetList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QWidgetList {
pub fn Append_1(&self) -> (/*void*/) {
    // QWidgetList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QWidgetList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QWidgetList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QWidgetList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QWidgetList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QWidgetList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QWidgetList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QWidgetList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QWidgetList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QWidgetList {
pub fn RemoveAll_0(&self) -> i32 {
    // QWidgetList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QWidgetList {
pub fn RemoveOne_0(&self) -> bool {
    // QWidgetList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QWidgetList {
pub fn TakeAt_0(&self) -> usize {
    // QWidgetList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QWidgetList {
pub fn TakeFirst_0(&self) -> usize {
    // QWidgetList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QWidgetList {
pub fn TakeLast_0(&self) -> usize {
    // QWidgetList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QWidgetList {
pub fn Move_0(&self) -> (/*void*/) {
    // QWidgetList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QWidgetList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QWidgetList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QWidgetList {
pub fn IndexOf_0(&self) -> i32 {
    // QWidgetList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QWidgetList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QWidgetList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QWidgetList {
pub fn Contains_0(&self) -> bool {
    // QWidgetList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QWidgetList {
pub fn Count_0(&self) -> i32 {
    // QWidgetList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QWidgetList {
pub fn Begin_0(&self) -> usize {
    // QWidgetList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QWidgetList {
pub fn Begin_1(&self) -> usize {
    // QWidgetList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QWidgetList {
pub fn Cbegin_0(&self) -> usize {
    // QWidgetList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QWidgetList {
pub fn ConstBegin_0(&self) -> usize {
    // QWidgetList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QWidgetList {
pub fn End_0(&self) -> usize {
    // QWidgetList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QWidgetList {
pub fn End_1(&self) -> usize {
    // QWidgetList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QWidgetList {
pub fn Cend_0(&self) -> usize {
    // QWidgetList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QWidgetList {
pub fn ConstEnd_0(&self) -> usize {
    // QWidgetList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QWidgetList {
pub fn Rbegin_0(&self) -> usize {
    // QWidgetList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QWidgetList {
pub fn Rend_0(&self) -> usize {
    // QWidgetList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QWidgetList {
pub fn Rbegin_1(&self) -> usize {
    // QWidgetList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QWidgetList {
pub fn Rend_1(&self) -> usize {
    // QWidgetList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QWidgetList {
pub fn Crbegin_0(&self) -> usize {
    // QWidgetList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QWidgetList {
pub fn Crend_0(&self) -> usize {
    // QWidgetList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QWidgetList {
pub fn Insert_1(&self) -> usize {
    // QWidgetList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QWidgetList {
pub fn Erase_0(&self) -> usize {
    // QWidgetList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QWidgetList {
pub fn Erase_1(&self) -> usize {
    // QWidgetList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QWidgetList {
pub fn Count_1(&self) -> i32 {
    // QWidgetList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QWidgetList {
pub fn Length_0(&self) -> i32 {
    // QWidgetList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QWidgetList {
pub fn First_0(&self) -> usize {
    // QWidgetList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QWidgetList {
pub fn ConstFirst_0(&self) -> usize {
    // QWidgetList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QWidgetList {
pub fn First_1(&self) -> usize {
    // QWidgetList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QWidgetList {
pub fn Last_0(&self) -> usize {
    // QWidgetList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QWidgetList {
pub fn Last_1(&self) -> usize {
    // QWidgetList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QWidgetList {
pub fn ConstLast_0(&self) -> usize {
    // QWidgetList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QWidgetList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QWidgetList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QWidgetList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QWidgetList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QWidgetList {
pub fn StartsWith_0(&self) -> bool {
    // QWidgetList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QWidgetList {
pub fn EndsWith_0(&self) -> bool {
    // QWidgetList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QWidgetList {
pub fn Mid_0(&self) -> usize {
    // QWidgetList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QWidgetList {
pub fn Value_0(&self) -> usize {
    // QWidgetList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QWidgetList {
pub fn Value_1(&self) -> usize {
    // QWidgetList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QWidgetList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QWidgetList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QWidgetList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QWidgetList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QWidgetList {
pub fn Front_0(&self) -> usize {
    // QWidgetList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QWidgetList {
pub fn Front_1(&self) -> usize {
    // QWidgetList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QWidgetList {
pub fn Back_0(&self) -> usize {
    // QWidgetList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QWidgetList {
pub fn Back_1(&self) -> usize {
    // QWidgetList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QWidgetList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QWidgetList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QWidgetList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QWidgetList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QWidgetList {
pub fn Empty_0(&self) -> bool {
    // QWidgetList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QWidgetList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QWidgetList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QWidgetList {
pub fn Operator_add_0(&self) -> usize {
    // QWidgetList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QWidgetList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QWidgetList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QWidgetList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QWidgetList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QWidgetList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QWidgetList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QWidgetList {
pub fn ToVector_0(&self) -> usize {
    // QWidgetList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QWidgetList {
pub fn ToSet_0(&self) -> usize {
    // QWidgetList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QWidgetList {
pub fn FromVector_0(&self) -> usize {
    // QWidgetList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QWidgetList {
pub fn FromSet_0(&self) -> usize {
    // QWidgetList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QWidgetList {
pub fn FromStdList_0(&self) -> usize {
    // QWidgetList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QWidgetList {
pub fn ToStdList_0(&self) -> i32 {
    // QWidgetList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QWidgetList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QWidgetList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QWidgetList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QWidgetList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QWidgetList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QWidgetList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QWidgetList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QWidgetList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QWidgetList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QWidgetList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QWidgetList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QWidgetList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QWidgetList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QWidgetList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QWidgetList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QWidgetList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QWidgetList {
pub fn IsValidIterator_0(&self) -> bool {
    // QWidgetList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QWidgetList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QWidgetList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QWidgetList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QWidgetList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QWidgetList {
pub fn Contains_impl_0(&self) -> bool {
    // QWidgetList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QWidgetList {
pub fn Contains_impl_1(&self) -> bool {
    // QWidgetList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QWidgetList {
pub fn Count_impl_0(&self) -> i32 {
    // QWidgetList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QWidgetList {
pub fn Count_impl_1(&self) -> i32 {
    // QWidgetList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QWidgetList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
