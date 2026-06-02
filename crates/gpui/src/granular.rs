//! Module for building [`Element`](elements) & [`crate::AnyView`](views) with fine grained control over what updates

use crate::{
    AnyElement, AnyEntity, AnyWeakEntity, App, Bounds, ContentMask, Context, Element, ElementId,
    Empty, Entity, EntityId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    PaintIndex, Pixels, PrepaintStateIndex, Render, Style, StyleRefinement, TextStyle, WeakEntity,
    Window,
};
use anyhow::Result;
use collections::FxHashSet;
use refineable::Refineable;

/// Train for Elements & Views that want to update granularly
pub trait Granular {
    //
}

/// Generic Granular [Element]/[View]
#[derive(Debug)]
pub struct AnyGranular {}

/// A Fragment of text that can be updated and placed within a [`GranularText`]
#[derive(Debug)]
pub struct TextFragment {}

impl IntoElement for TextFragment {
    type Element = GranularText;

    fn into_element(self) -> Self::Element {
        GranularText::from_fragment(self)
    }
}

/// An element that renders text that can be granularly updated
#[derive(Debug)]
pub struct GranularText {}

impl GranularText {
    /// Creates a new empty [`GranularText`]
    pub fn new() -> Self {
        todo!();
    }

    /// Creates a new [`GranularText`] from a fragment
    pub(crate) fn from_fragment(frag: TextFragment) -> Self {
        let _ = frag;
        todo!();
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
