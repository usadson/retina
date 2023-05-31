// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The cascade origin specifies where a certain rule, declaration, or
/// stylesheet originated from, which is important for ordering the applicable
/// rules for a given node. As an example, the user-agent stylesheet might
/// default the [foreground color] to [black], but the author stylesheet
/// wants to use the color [red]. Because the author cascade origin precedes
/// user-agent origin, the final computed foreground color is red.
///
/// __Specification__
/// > Each style rule has a [***cascade origin***], which determines where it
/// > enters the cascade.
///
/// # References
/// * [CSS - Cascading and Inheritance Level 5 § 6.2. Cascading Origin][cascade-origin]
///
/// [black]: https://drafts.csswg.org/css-color-3/#black
/// [***cascade origin***]: https://drafts.csswg.org/css-cascade-5/#cascading-origins
/// [foreground color]: https://drafts.csswg.org/css-color-3/#foreground
/// [red]: [black]: https://drafts.csswg.org/css-color-3/#black
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CascadeOrigin {
    /// # [Author Origin]
    /// The author specifies style sheets for a source document according to the
    /// conventions of the document language. For instance, in HTML, style
    /// sheets may be included in the document or linked externally.
    ///
    /// [Author Origin]: https://drafts.csswg.org/css-cascade-5/#cascade-origin-author
    Author,

    /// # [User Origin]
    /// The user may be able to specify style information for a particular
    /// document. For example, the user may specify a file that contains a style
    /// sheet or the user agent may provide an interface that generates a user
    /// style sheet (or behaves as if it did).
    ///
    /// [User Origin]: https://drafts.csswg.org/css-cascade-5/#cascade-origin-user
    User,

    /// # [User-Agent Origin]
    /// Conforming user agents must apply a default style sheet (or behave as if
    /// they did). A user agent’s default style sheet should present the
    /// elements of the document language in ways that satisfy general
    /// presentation expectations for the document language (e.g., for visual
    /// browsers, the EM element in HTML is presented using an italic font). See
    /// e.g. the [HTML user agent style sheet].
    ///
    /// [User-Agent Origin]: https://drafts.csswg.org/css-cascade-5/#cascade-origin-ua
    /// [HTML user agent style sheet]: https://html.spec.whatwg.org/multipage/rendering.html#the-css-user-agent-style-sheet-and-presentational-hints
    UserAgent,
}
