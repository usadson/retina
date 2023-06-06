// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// The `DumpableNode` trait allows a streamlined dumping interface for
/// node-tree structures. Dumping is useful for debugging scenarios where it
/// isn't easy or practical to just inspect the process with a debugger.
///
/// It also tries to be distinct from the [`Debug`] trait, which shouldn't be
/// used to write large, complex tree structures.
pub trait DumpableNode {
    /// A convenience function for dumping [this object][Self] to the standard
    /// output (`stdout`) without propagating errors.
    #[inline]
    fn dump(&self) {
        _ = self.dump_to_stdout()
    }

    /// Dump [this object][Self] to the given [`writer`][std::io::Write], with
    /// a `depth` indicating how far the object is in the tree, with 0 being the
    /// root node.
    fn dump_to(&self, depth: usize, writer: &mut dyn std::io::Write) -> Result<(), std::io::Error>;

    /// Dump [this object][Self] to the standard output (`stdout`).
    #[inline]
    fn dump_to_stdout(&self) -> Result<(), std::io::Error> {
        self.dump_to(0, &mut std::io::stdout())
    }
}
