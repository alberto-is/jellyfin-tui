//! Tests for the non-obvious decisions of the generic select-mode state machine (src/select.rs).
//! The rest is a thin HashSet wrapper not worth pinning down.

use crate::select::{SelectMode, SelectPane};

#[test]
fn enter_ignores_an_empty_seed() {
    let mut select = SelectMode::default();
    select.enter(SelectPane::PlaylistTracks, Some(String::new()));
    assert!(select.is_active());
    assert!(select.is_empty());
}

#[test]
fn entering_a_new_session_drops_the_previous_selection() {
    let mut select = SelectMode::default();
    select.enter(SelectPane::PlaylistTracks, Some("old".to_string()));
    select.enter(SelectPane::PlaylistTracks, None);
    assert!(!select.is_selected("old"));
    assert!(select.is_empty());
}

#[test]
fn toggle_is_a_no_op_while_inactive() {
    let mut select = SelectMode::default();
    select.toggle("a".to_string());
    assert!(select.is_empty());
}

#[test]
fn toggle_ignores_empty_keys() {
    let mut select = SelectMode::default();
    select.enter(SelectPane::PlaylistTracks, None);
    select.toggle(String::new());
    assert!(select.is_empty());
}
