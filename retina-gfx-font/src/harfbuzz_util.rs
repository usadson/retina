// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use harfbuzz_rs::Tag;

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-calt>
pub const TAG_CONTEXTUAL_ALTERNATIVES: Tag = Tag::new('c', 'a', 'l', 't');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-clig>
pub const TAG_CONTEXTUAL_LIGATURES: Tag = Tag::new('c', 'l', 'i', 'g');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-dlig>
pub const TAG_DISCRETIONARY_LIGATURES: Tag = Tag::new('d', 'l', 'i', 'g');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-hlig>
pub const TAG_HISTORICAL_LIGATURES: Tag = Tag::new('h', 'l', 'i', 'g');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ko#tag-kern>
pub const TAG_KERN: Tag = Tag::new('k', 'e', 'r', 'n');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-liga>
pub const TAG_STANDARD_LIGATURES: Tag = Tag::new('l', 'i', 'g', 'a');
