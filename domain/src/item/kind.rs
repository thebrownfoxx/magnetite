use std::rc::Rc;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ItemKindId(Rc<str>);

impl ItemKindId {
    pub fn new(value: impl Into<Rc<str>>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for ItemKindId
where
    T: Into<Rc<str>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for ItemKindId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<ItemKindId> for ItemKindId {
    fn as_ref(&self) -> &ItemKindId {
        self
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct ItemKind {
    id: ItemKindId,
    name: Rc<str>,
    is_book: bool,
}

impl ItemKind {
    pub fn new_book(id: impl Into<ItemKindId>, name: impl Into<Rc<str>>) -> Self {
        Self::new(id, name, true)
    }

    pub fn new_non_book(id: impl Into<ItemKindId>, name: impl Into<Rc<str>>) -> Self {
        Self::new(id, name, false)
    }

    fn new(id: impl Into<ItemKindId>, name: impl Into<Rc<str>>, is_book: bool) -> Self {
        Self { id: id.into(), name: name.into(), is_book }
    }

    pub fn id(&self) -> &ItemKindId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_book(&self) -> bool {
        self.is_book
    }
}

impl AsRef<ItemKindId> for ItemKind {
    fn as_ref(&self) -> &ItemKindId {
        self.id()
    }
}
