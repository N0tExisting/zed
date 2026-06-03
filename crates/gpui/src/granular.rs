//! Module for building [`Element`](elements) & [`crate::AnyView`](views)
//! with fine grained updates that only rerender what changed

use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, ContentMask, Context, Element, ElementId,
    Empty, Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    PaintIndex, Pixels, PrepaintStateIndex, Render, SharedString, Style, StyleRefinement,
    TextStyle, WeakEntity, Window,
};
use anyhow::Result;
use collections::{FxBuildHasher, FxHashSet, IndexMap};
use refineable::Refineable;
use slotmap::{HopSlotMap, Key, SecondaryMap, new_key_type};
use std::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// Train for Elements & Views that want to update granularly
pub trait Granular {
    //
}

/// Generic Granular [Element]/[View]
#[derive(Debug)]
pub struct AnyGranular {}

new_key_type! { pub struct TextKey; }

/// A Fragment of text that can be updated and placed within a [`GranularText`]
#[derive(Debug)]
pub struct TextFragment {
    text: SharedString,
}

impl TextFragment {
    /// Update the text and notify the Element
    pub fn update(&mut self, new_text: SharedString) {
        self.text = new_text;
        // TODO: Inform the owning GranularText
    }
}

impl IntoElement for TextFragment {
    type Element = GranularText;

    fn into_element(self) -> Self::Element {
        GranularText::from_fragment(self)
    }
}

/// A generic text for [`GranularText`]
#[derive(Debug)]
pub enum GranularString {
    /// An updatable fragment
    Fragment(TextFragment),
    /// A static string, cant be updated
    Static(SharedString),
}

impl GranularString {
    /// Get the underlying string
    pub fn str(&self) -> &str {
        match self {
            Self::Fragment(frag) => &frag.text,
            Self::Static(str) => str,
        }
    }
}

impl From<TextFragment> for GranularString {
    fn from(value: TextFragment) -> Self {
        Self::Fragment(value)
    }
}

impl From<SharedString> for GranularString {
    fn from(value: SharedString) -> Self {
        Self::Static(value)
    }
}

impl From<&str> for GranularString {
    fn from(value: &str) -> Self {
        Self::Static(value.into())
    }
}

impl From<String> for GranularString {
    fn from(value: String) -> Self {
        Self::Static(value.into())
    }
}

#[derive(Debug, Default)]
struct InnerGranularText(Vec<GranularString>);

impl InnerGranularText {
    fn to_single_string(&self) -> String {
        self.0.iter().fold("".into(), |text, str| text + str.str())
    }
}

impl From<Vec<GranularString>> for InnerGranularText {
    fn from(strings: Vec<GranularString>) -> Self {
        Self(strings)
    }
}

impl Deref for InnerGranularText {
    type Target = Vec<GranularString>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InnerGranularText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An element that renders text that can be granularly updated
#[derive(Debug)]
pub struct GranularText {
    inner: InnerGranularText,
    cache: String,
    // Should this be a `Cell`?
    updated: AtomicBool,
}

impl Default for GranularText {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            cache: Default::default(),
            updated: AtomicBool::new(true),
        }
    }
}

impl GranularText {
    /// Creates a new empty [`GranularText`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`GranularText`] from a fragment
    pub(crate) fn from_fragment(frag: TextFragment) -> Self {
        Self {
            inner: vec![frag.into()].into(),
            ..Default::default()
        }
    }

    /// Add a fragment to the end of the string
    pub fn add_fragment(str: impl Into<SharedString>) -> usize {
        let _ = str;
        todo!()
    }

    /// All of the stuff
    // TODO: This is wrong probably?
    pub fn text(&self) -> &[GranularString] {
        self.inner.as_slice()
    }

    fn rebuild_cache(&mut self) -> bool {
        let updated = self.updated.swap(false, Ordering::AcqRel);
        if updated {
            self.cache = self.inner.to_single_string();
        }
        updated
    }
}

impl Element for GranularText {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        todo!()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // TODO: This should be smarter (because it is the first time)
        //? Or should we just build the cache as we add Strings?
        let _updated = self.rebuild_cache();
        //window.request_measured_layout(style, measure);
        todo!()
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let _updated = self.rebuild_cache();
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        todo!()
    }
}

impl IntoElement for GranularText {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A group Of [`Element`](elements)/[`crate::AnyView`](views) without an actual parent element
#[derive(Debug)]
pub struct ElementFragment {}

/// An Element that can dynamically gain and lose children
#[derive(Debug)]
pub struct GranularChildren {}

#[inline]
fn assert_none<T: std::fmt::Debug>(opt: Option<T>) {
    //assert_matches!(opt, None);
    assert!(opt.is_none(), "Expected None, but got {:?}", opt);
}

#[derive(Debug)]
struct GranularStore<K: Key, V, C> {
    map: HopSlotMap<K, V>,
    index: IndexMap<K, bool>,
    cache: SecondaryMap<K, C>,
}

impl<K: Key, V, C: std::fmt::Debug> GranularStore<K, V, C> {
    //type KV = (K, V);
    //type KI = (K, usize);
    //type IV = (usize, V);
    //type VC = (V, C);
    //type VDC = (V, bool, C);
    //type VIDC = (V, usize, bool, C);
    //type KVIDC = (K, V, usize, bool, C);

    fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HopSlotMap::with_capacity_and_key(capacity),
            index: IndexMap::with_capacity_and_hasher(capacity, FxBuildHasher),
            cache: SecondaryMap::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: V, cache: C) -> (K, usize) {
        let key = self.map.insert(value);
        let (index, val) = self.index.insert_full(key, true);
        assert_none(val);
        assert_none(self.cache.insert(key, cache));
        (key, index)
    }

    fn insert_at(&mut self, value: V, cache: C) -> K {
        //self.index.shift_insert;
        //self.index.insert_before;
        todo!()
    }

    fn remove(&mut self, key: K) -> Option<(K, V, usize, bool, C)> {
        let value = self.map.remove(key)?;
        let (index, _k, dirty) = self.index.shift_remove_full(&key).unwrap();
        let cache = self.cache.remove(key).unwrap();

        Some((key, value, index, dirty, cache))
    }

    fn swap_indices(
        &mut self,
        a: &K,
        b: &K,
    ) -> Result<(usize, usize), (Option<usize>, Option<usize>)> {
        let i_a = self.index.get_index_of(a);
        let i_b = self.index.get_index_of(b);

        if i_a.is_none() || i_b.is_none() {
            return Err((i_a, i_b));
        }

        let i_a = i_a.unwrap();
        let i_b = i_b.unwrap();

        self.index.swap_indices(i_a, i_b);

        Ok((i_a, i_b))
    }

    fn move_index(&mut self, key: &K, to: usize) -> Result<()> {
        let i_from = self.index.get_index_of(key);
        anyhow::ensure!(i_from.is_some(), "Key does not exist");
        let i_from = i_from.unwrap();

        self.index.move_index(i_from, to);
        Ok(())
    }

    fn get_index(&self, key: &K) -> Option<usize> {
        self.index.get_index_of(key)
    }

    fn remajigger(&mut self) {
        self.index.last();
    }
}

impl<C> GranularStore<TextKey, GranularString, C> {
    fn to_single_string(&self) -> String {
        self.index
            .iter()
            .map(|(key, _)| self.map.get(key.clone()))
            .filter_map(std::convert::identity)
            .fold("".into(), |text, str| text + str.str())
    }
}
