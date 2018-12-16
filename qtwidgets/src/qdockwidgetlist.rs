
// mod ::widgets::QDockWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qdockwidget.h
// #include <qdockwidget.h>
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
pub struct QDockWidgetList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_equal_0(&self) -> usize {
    // QDockWidgetList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QDockWidgetList {
pub fn Operator_equal_1(&self) -> usize {
    // QDockWidgetList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QDockWidgetList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QDockWidgetList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QDockWidgetList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QDockWidgetList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QDockWidgetList {
pub fn Size_0(&self) -> i32 {
    // QDockWidgetList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QDockWidgetList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QDockWidgetList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QDockWidgetList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QDockWidgetList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QDockWidgetList {
pub fn IsDetached_0(&self) -> bool {
    // QDockWidgetList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QDockWidgetList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QDockWidgetList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QDockWidgetList {
pub fn IsSharedWith_0(&self) -> bool {
    // QDockWidgetList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QDockWidgetList {
pub fn IsEmpty_0(&self) -> bool {
    // QDockWidgetList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QDockWidgetList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QDockWidgetList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QDockWidgetList {
pub fn At_0(&self) -> usize {
    // QDockWidgetList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QDockWidgetList {
pub fn Operator_get_index_0(&self) -> usize {
    // QDockWidgetList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QDockWidgetList {
pub fn Operator_get_index_1(&self) -> usize {
    // QDockWidgetList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QDockWidgetList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QDockWidgetList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QDockWidgetList {
pub fn Append_0(&self) -> (/*void*/) {
    // QDockWidgetList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QDockWidgetList {
pub fn Append_1(&self) -> (/*void*/) {
    // QDockWidgetList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QDockWidgetList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QDockWidgetList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QDockWidgetList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QDockWidgetList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QDockWidgetList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QDockWidgetList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QDockWidgetList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QDockWidgetList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QDockWidgetList {
pub fn RemoveAll_0(&self) -> i32 {
    // QDockWidgetList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QDockWidgetList {
pub fn RemoveOne_0(&self) -> bool {
    // QDockWidgetList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QDockWidgetList {
pub fn TakeAt_0(&self) -> usize {
    // QDockWidgetList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QDockWidgetList {
pub fn TakeFirst_0(&self) -> usize {
    // QDockWidgetList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QDockWidgetList {
pub fn TakeLast_0(&self) -> usize {
    // QDockWidgetList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QDockWidgetList {
pub fn Move_0(&self) -> (/*void*/) {
    // QDockWidgetList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QDockWidgetList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QDockWidgetList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QDockWidgetList {
pub fn IndexOf_0(&self) -> i32 {
    // QDockWidgetList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QDockWidgetList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QDockWidgetList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QDockWidgetList {
pub fn Contains_0(&self) -> bool {
    // QDockWidgetList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QDockWidgetList {
pub fn Count_0(&self) -> i32 {
    // QDockWidgetList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QDockWidgetList {
pub fn Begin_0(&self) -> usize {
    // QDockWidgetList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QDockWidgetList {
pub fn Begin_1(&self) -> usize {
    // QDockWidgetList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QDockWidgetList {
pub fn Cbegin_0(&self) -> usize {
    // QDockWidgetList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QDockWidgetList {
pub fn ConstBegin_0(&self) -> usize {
    // QDockWidgetList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QDockWidgetList {
pub fn End_0(&self) -> usize {
    // QDockWidgetList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QDockWidgetList {
pub fn End_1(&self) -> usize {
    // QDockWidgetList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QDockWidgetList {
pub fn Cend_0(&self) -> usize {
    // QDockWidgetList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QDockWidgetList {
pub fn ConstEnd_0(&self) -> usize {
    // QDockWidgetList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QDockWidgetList {
pub fn Rbegin_0(&self) -> usize {
    // QDockWidgetList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QDockWidgetList {
pub fn Rend_0(&self) -> usize {
    // QDockWidgetList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QDockWidgetList {
pub fn Rbegin_1(&self) -> usize {
    // QDockWidgetList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QDockWidgetList {
pub fn Rend_1(&self) -> usize {
    // QDockWidgetList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QDockWidgetList {
pub fn Crbegin_0(&self) -> usize {
    // QDockWidgetList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QDockWidgetList {
pub fn Crend_0(&self) -> usize {
    // QDockWidgetList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QDockWidgetList {
pub fn Insert_1(&self) -> usize {
    // QDockWidgetList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QDockWidgetList {
pub fn Erase_0(&self) -> usize {
    // QDockWidgetList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QDockWidgetList {
pub fn Erase_1(&self) -> usize {
    // QDockWidgetList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QDockWidgetList {
pub fn Count_1(&self) -> i32 {
    // QDockWidgetList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QDockWidgetList {
pub fn Length_0(&self) -> i32 {
    // QDockWidgetList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QDockWidgetList {
pub fn First_0(&self) -> usize {
    // QDockWidgetList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QDockWidgetList {
pub fn ConstFirst_0(&self) -> usize {
    // QDockWidgetList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QDockWidgetList {
pub fn First_1(&self) -> usize {
    // QDockWidgetList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QDockWidgetList {
pub fn Last_0(&self) -> usize {
    // QDockWidgetList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QDockWidgetList {
pub fn Last_1(&self) -> usize {
    // QDockWidgetList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QDockWidgetList {
pub fn ConstLast_0(&self) -> usize {
    // QDockWidgetList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QDockWidgetList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QDockWidgetList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QDockWidgetList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QDockWidgetList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QDockWidgetList {
pub fn StartsWith_0(&self) -> bool {
    // QDockWidgetList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QDockWidgetList {
pub fn EndsWith_0(&self) -> bool {
    // QDockWidgetList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QDockWidgetList {
pub fn Mid_0(&self) -> usize {
    // QDockWidgetList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QDockWidgetList {
pub fn Value_0(&self) -> usize {
    // QDockWidgetList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QDockWidgetList {
pub fn Value_1(&self) -> usize {
    // QDockWidgetList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QDockWidgetList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QDockWidgetList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QDockWidgetList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QDockWidgetList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QDockWidgetList {
pub fn Front_0(&self) -> usize {
    // QDockWidgetList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QDockWidgetList {
pub fn Front_1(&self) -> usize {
    // QDockWidgetList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QDockWidgetList {
pub fn Back_0(&self) -> usize {
    // QDockWidgetList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QDockWidgetList {
pub fn Back_1(&self) -> usize {
    // QDockWidgetList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QDockWidgetList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QDockWidgetList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QDockWidgetList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QDockWidgetList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QDockWidgetList {
pub fn Empty_0(&self) -> bool {
    // QDockWidgetList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QDockWidgetList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_add_0(&self) -> usize {
    // QDockWidgetList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QDockWidgetList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QDockWidgetList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QDockWidgetList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QDockWidgetList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QDockWidgetList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QDockWidgetList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QDockWidgetList {
pub fn ToVector_0(&self) -> usize {
    // QDockWidgetList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QDockWidgetList {
pub fn ToSet_0(&self) -> usize {
    // QDockWidgetList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QDockWidgetList {
pub fn FromVector_0(&self) -> usize {
    // QDockWidgetList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QDockWidgetList {
pub fn FromSet_0(&self) -> usize {
    // QDockWidgetList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QDockWidgetList {
pub fn FromStdList_0(&self) -> usize {
    // QDockWidgetList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QDockWidgetList {
pub fn ToStdList_0(&self) -> i32 {
    // QDockWidgetList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QDockWidgetList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QDockWidgetList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QDockWidgetList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QDockWidgetList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QDockWidgetList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QDockWidgetList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QDockWidgetList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QDockWidgetList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QDockWidgetList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QDockWidgetList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QDockWidgetList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QDockWidgetList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QDockWidgetList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QDockWidgetList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QDockWidgetList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QDockWidgetList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QDockWidgetList {
pub fn IsValidIterator_0(&self) -> bool {
    // QDockWidgetList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QDockWidgetList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QDockWidgetList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Contains_impl_0(&self) -> bool {
    // QDockWidgetList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Contains_impl_1(&self) -> bool {
    // QDockWidgetList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Count_impl_0(&self) -> i32 {
    // QDockWidgetList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QDockWidgetList {
pub fn Count_impl_1(&self) -> i32 {
    // QDockWidgetList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QDockWidgetList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
