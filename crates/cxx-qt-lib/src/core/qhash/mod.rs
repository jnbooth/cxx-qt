// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

/// The QHash class is a template class that provides a hash-table-based dictionary.
///
/// Note that this means that T needs to have a C++ global
/// [`qHash()` function](https://doc.qt.io/qt-6/qhash.html#qhash).
///
/// To use QHash with a custom pair, implement the [`QHashPair`] trait for T.
#[repr(C)]
pub struct QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    _space: MaybeUninit<usize>,
    _key: PhantomData<Key>,
    _value: PhantomData<Value>,
}

impl<Key, Value> Clone for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        <Self as QHashPair<Key, Value>>::clone(self)
    }
}

impl<Key, Value> Default for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    /// Constructs an empty hash.
    fn default() -> Self {
        <Self as QHashPair<Key, Value>>::default()
    }
}

impl<Key, Value> Drop for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    /// Destroys the hash.
    fn drop(&mut self) {
        <Self as QHashPair<Key, Value>>::drop(self)
    }
}

impl<Key, Value> PartialEq for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
    Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|(k, v)| other.get(k).as_ref() == Some(v))
    }
}

impl<Key, Value> Eq for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
    Value: Eq,
{
}

impl<Key, Value> QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    /// Removes all items from the hash.
    pub fn clear(&mut self) {
        <Self as QHashPair<Key, Value>>::clear(self)
    }

    /// Returns true if the hash contains an item with the key; otherwise returns false.
    pub fn contains(&self, key: &Key) -> bool {
        <Self as QHashPair<Key, Value>>::contains(self, key)
    }

    /// Returns the value associated with the key if it exists.
    pub fn get(&self, key: &Key) -> Option<Value> {
        if self.contains(key) {
            Some(<Self as QHashPair<Key, Value>>::get_or_default(self, key))
        } else {
            None
        }
    }

    /// Returns the value associated with the key or a default value.
    pub fn get_or_default(&self, key: &Key) -> Value {
        <Self as QHashPair<Key, Value>>::get_or_default(self, key)
    }

    /// Inserts a new item with the key and a value of value.
    ///
    /// The key and value is a reference here so it can be opaque or trivial but
    /// note that the key and value is copied when being inserted into the hash.
    pub fn insert_clone(&mut self, key: &Key, value: &Value) {
        <Self as QHashPair<Key, Value>>::insert_clone(self, key, value)
    }

    /// Returns true if the hash contains no items; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        <Self as QHashPair<Key, Value>>::len(self) == 0
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is (&'a T::Key, &'a T::Value).
    pub fn iter(&self) -> Iter<Key, Value> {
        Iter {
            hash: self,
            index: 0,
        }
    }

    /// Returns the number of items in the hash.
    pub fn len(&self) -> isize {
        <Self as QHashPair<Key, Value>>::len(self)
    }

    /// Removes all the items that have the key from the hash.
    ///
    /// Returns true if at least one item was removed, otherwise returns false.
    pub fn remove(&mut self, key: &Key) -> bool {
        <Self as QHashPair<Key, Value>>::remove(self, key)
    }
}

impl<Key, Value> QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
    Key: ExternType<Kind = cxx::kind::Trivial>,
    Value: ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts a new item with the key and a value of value.
    pub fn insert(&mut self, key: Key, value: Value) {
        <Self as QHashPair<Key, Value>>::insert(self, key, value)
    }
}

unsafe impl<Key, Value> ExternType for QHash<Key, Value>
where
    Self: QHashPair<Key, Value>,
{
    type Id = <Self as QHashPair<Key, Value>>::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, Key, Value>
where
    QHash<Key, Value>: QHashPair<Key, Value>,
{
    hash: &'a QHash<Key, Value>,
    index: isize,
}

impl<'a, Key, Value> Iterator for Iter<'a, Key, Value>
where
    QHash<Key, Value>: QHashPair<Key, Value>,
{
    type Item = (&'a Key, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.hash.len() {
            let next = unsafe {
                (
                    <QHash<Key, Value>>::get_unchecked_key(self.hash, self.index),
                    <QHash<Key, Value>>::get_unchecked_value(self.hash, self.index),
                )
            };
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, Key, Value> ExactSizeIterator for Iter<'a, Key, Value>
where
    QHash<Key, Value>: QHashPair<Key, Value>,
{
    fn len(&self) -> usize {
        (self.hash.len() - self.index) as usize
    }
}

/// Trait implementation for a pair in a [`QHash`].
pub trait QHashPair<Key, Value>: Sized
where
    QHash<Key, Value>: QHashPair<Key, Value>,
{
    type TypeId;

    fn clear(hash: &mut QHash<Key, Value>);
    fn clone(hash: &QHash<Key, Value>) -> QHash<Key, Value>;
    fn contains(hash: &QHash<Key, Value>, key: &Key) -> bool;
    fn default() -> QHash<Key, Value>;
    fn drop(hash: &mut QHash<Key, Value>);
    fn get_or_default(hash: &QHash<Key, Value>, key: &Key) -> Value;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_key(hash: &QHash<Key, Value>, pos: isize) -> &Key;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_value(hash: &QHash<Key, Value>, pos: isize) -> &Value;
    fn insert(hash: &mut QHash<Key, Value>, key: Key, value: Value)
    where
        Key: ExternType<Kind = cxx::kind::Trivial>,
        Value: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(hash: &mut QHash<Key, Value>, key: &Key, value: &Value);
    fn len(hash: &QHash<Key, Value>) -> isize;
    fn remove(hash: &mut QHash<Key, Value>, key: &Key) -> bool;
}

macro_rules! impl_qhash_pair {
    ( $keyTypeName:ty, $valueTypeName:ty, $module:ident, $typeId:literal ) => {
        mod $module;

        impl QHashPair<$keyTypeName, $valueTypeName> for QHash<$keyTypeName, $valueTypeName> {
            type TypeId = type_id!($typeId);

            fn clear(hash: &mut Self) {
                hash.cxx_clear();
            }

            fn clone(hash: &Self) -> Self {
                $module::clone(hash)
            }

            fn contains(hash: &Self, key: &$keyTypeName) -> bool {
                hash.cxx_contains(key)
            }

            fn default() -> Self {
                $module::default()
            }

            fn drop(hash: &mut Self) {
                $module::drop(hash);
            }

            fn get_or_default(hash: &Self, key: &$keyTypeName) -> $valueTypeName {
                $module::get_or_default(hash, key)
            }

            unsafe fn get_unchecked_key(hash: &Self, pos: isize) -> &$keyTypeName {
                $module::get_unchecked_key(hash, pos)
            }

            unsafe fn get_unchecked_value(hash: &Self, pos: isize) -> &$valueTypeName {
                $module::get_unchecked_value(hash, pos)
            }

            fn insert(hash: &mut Self, key: $keyTypeName, value: $valueTypeName) {
                $module::insert(hash, &key, &value);
            }

            fn insert_clone(hash: &mut Self, key: &$keyTypeName, value: &$valueTypeName) {
                $module::insert(hash, key, value);
            }

            fn len(hash: &Self) -> isize {
                $module::len(hash)
            }

            fn remove(hash: &mut Self, key: &$keyTypeName) -> bool {
                $module::remove(hash, key)
            }
        }
    };
}

// For now we will implement useful combinations for Qt
// Other combinations the developer will have to implement themselves
// or a generator could be made later https://github.com/KDAB/cxx-qt/issues/355
//
// QVariantHash
impl_qhash_pair!(
    crate::QString,
    crate::QVariant,
    qhash_qstring_qvariant,
    "QHash_QString_QVariant"
);
// QHash<int, QByteArray> which is used for QAbstractItemModel::roleNames
impl_qhash_pair!(
    i32,
    crate::QByteArray,
    qhash_i32_qbytearray,
    "QHash_i32_QByteArray"
);
