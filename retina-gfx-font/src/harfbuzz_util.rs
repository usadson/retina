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

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-pcap>
pub const TAG_PETITE_CAPITALS: Tag = Tag::new('p', 'c', 'a', 'p');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-c2pc>
pub const TAG_PETITE_CAPITALS_FROM_CAPITALS: Tag = Tag::new('c', '2', 'p', 'c');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-smpc>
pub const TAG_SMALL_CAPITALS: Tag = Tag::new('s', 'm', 'p', 'c');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-c2sc>
pub const TAG_SMALL_CAPITALS_FROM_CAPITALS: Tag = Tag::new('c', '2', 's', 'c');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-liga>
pub const TAG_STANDARD_LIGATURES: Tag = Tag::new('l', 'i', 'g', 'a');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-titl>
pub const TAG_TITLING: Tag = Tag::new('t', 'i', 't', 'l');

/// <https://learn.microsoft.com/en-us/typography/opentype/spec/features_ae#tag-unic>
pub const TAG_UNICASE: Tag = Tag::new('u', 'n', 'i', 'c');
