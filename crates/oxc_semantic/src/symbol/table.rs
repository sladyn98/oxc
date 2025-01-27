use std::ops::{Deref, Index, IndexMut};

use oxc_ast::{Atom, Span};

use super::reference::ResolvedReferenceId;
use super::{Symbol, SymbolFlags, SymbolId};
use crate::node::AstNodeId;
use crate::{Reference, ResolvedReference};

/// `SymbolTable` is a storage of all the symbols (related to `BindingIdentifiers`)
/// and references (related to `IdentifierReferences`) of the program. It supports two
/// kinds of queries: indexing by `SymbolId` retrieves the corresponding `Symbol` and
/// indexing by `ResolvedReferenceId` retrieves the correspodning `ResolvedReference`
///
#[derive(Debug, Default)]
pub struct SymbolTable {
    /// Stores all the `Symbols` indexed by `SymbolId`
    symbols: Vec<Symbol>,
    /// Stores all the resolved references indexed by `ResolvedReferenceId`
    resolved_references: Vec<ResolvedReference>,
}

impl Index<SymbolId> for SymbolTable {
    type Output = Symbol;

    fn index(&self, index: SymbolId) -> &Self::Output {
        &self.symbols[index.index0()]
    }
}

impl IndexMut<SymbolId> for SymbolTable {
    fn index_mut(&mut self, index: SymbolId) -> &mut Self::Output {
        &mut self.symbols[index.index0()]
    }
}

impl Index<ResolvedReferenceId> for SymbolTable {
    type Output = ResolvedReference;

    fn index(&self, index: ResolvedReferenceId) -> &Self::Output {
        &self.resolved_references[index.index0()]
    }
}

impl IndexMut<ResolvedReferenceId> for SymbolTable {
    fn index_mut(&mut self, index: ResolvedReferenceId) -> &mut Self::Output {
        &mut self.resolved_references[index.index0()]
    }
}

impl Deref for SymbolTable {
    type Target = Vec<Symbol>;

    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}

impl SymbolTable {
    #[must_use]
    pub fn symbols(&self) -> &Vec<Symbol> {
        &self.symbols
    }

    #[must_use]
    pub fn get_symbol(&self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(id.index0())
    }

    #[must_use]
    pub(crate) fn create(
        &mut self,
        declaration: AstNodeId,
        name: Atom,
        span: Span,
        flags: SymbolFlags,
    ) -> SymbolId {
        let symbol_id = SymbolId::new(self.symbols.len() + 1);
        let symbol = Symbol::new(symbol_id, declaration, name, span, flags);
        self.symbols.push(symbol);
        symbol_id
    }

    #[must_use]
    pub fn resolved_references(&self) -> &Vec<ResolvedReference> {
        &self.resolved_references
    }

    #[must_use]
    pub fn get_resolved_reference(&self, id: ResolvedReferenceId) -> Option<&ResolvedReference> {
        self.resolved_references.get(id.index0())
    }

    /// Resolve all `references` to `symbol_id`
    pub(crate) fn resolve_reference(&mut self, references: Vec<Reference>, symbol_id: SymbolId) {
        let additional_len = references.len();
        let symbol = &mut self.symbols[symbol_id];

        self.resolved_references.reserve(additional_len);
        symbol.references.reserve(additional_len);

        for reference in references {
            let resolved_reference_id =
                ResolvedReferenceId::new(self.resolved_references.len() + 1);
            let resolved_reference = reference.resolve_to(symbol_id);
            self.resolved_references.push(resolved_reference);
            // explicitly push to vector here in correspondence to the previous reserve call
            symbol.references.push(resolved_reference_id);
        }
    }
}
