use std::io::Write;

use typed_arena::Arena;

use crate::backend::latex::{self, preamble};
use crate::backend::Backend;
use crate::config::Config;
use crate::diagnostics::Diagnostics;
use crate::error::FatalResult;

#[derive(Debug)]
pub struct Thesis;

#[rustfmt::skip]
impl<'a> Backend<'a> for Thesis {
    type Text = latex::TextGen;
    type Latex = latex::LatexGen;
    type FootnoteReference = latex::FootnoteReferenceGen;
    type BiberReferences = latex::BiberReferencesGen;
    type Url = latex::UrlGen;
    type InterLink = latex::InterLinkGen;
    type Image = latex::ImageGen;
    type Svg = latex::SvgGen;
    type Label = latex::LabelGen;
    type Pdf = latex::PdfGen;
    type SoftBreak = latex::SoftBreakGen;
    type HardBreak = latex::HardBreakGen;
    type TaskListMarker = latex::TaskListMarkerGen;
    type TableOfContents = latex::TableOfContentsGen;
    type Bibliography = latex::BibliographyGen;
    type ListOfTables = latex::ListOfTablesGen;
    type ListOfFigures = latex::ListOfFiguresGen;
    type ListOfListings = latex::ListOfListingsGen;
    type Appendix = latex::AppendixGen;

    type Paragraph = latex::ParagraphGen;
    type Rule = latex::RuleGen;
    type Header = latex::BookHeaderGen<'a>;
    type BlockQuote = latex::BlockQuoteGen;
    type CodeBlock = latex::CodeBlockGen;
    type List = latex::ListGen;
    type Enumerate = latex::EnumerateGen;
    type Item = latex::ItemGen;
    type FootnoteDefinition = latex::FootnoteDefinitionGen;
    type UrlWithContent = latex::UrlWithContentGen<'a>;
    type InterLinkWithContent = latex::InterLinkWithContentGen;
    type HtmlBlock = latex::HtmlBlockGen;
    type Figure = latex::FigureGen<'a>;

    type TableFigure = latex::TableFigureGen<'a>;
    type Table = latex::TableGen<'a>;
    type TableHead = latex::TableHeadGen;
    type TableRow = latex::TableRowGen;
    type TableCell = latex::TableCellGen;

    type InlineEmphasis = latex::InlineEmphasisGen;
    type InlineStrong = latex::InlineStrongGen;
    type InlineStrikethrough = latex::InlineStrikethroughGen;
    type InlineCode = latex::InlineCodeGen;
    type InlineMath = latex::InlineMathGen;

    type Equation = latex::EquationGen<'a>;
    type NumberedEquation = latex::NumberedEquationGen<'a>;
    type Graphviz = latex::GraphvizGen<'a>;

    fn new() -> Self {
        Thesis
    }

    fn gen_preamble(&mut self, cfg: &Config, out: &mut impl Write, diagnostics: &Diagnostics<'a>) -> FatalResult<()> {
        // TODO: itemizespacing
        preamble::write_documentclass(cfg, out, "scrbook", "headsepline,footsepline,BCOR=12mm,DIV=12,")?;
        preamble::write_packages(cfg, out)?;
        preamble::write_fixes(cfg, out)?;

        writeln!(out)?;
        writeln!(out, "\\begin{{document}}")?;
        writeln!(out)?;

        preamble::write_manual_titlepage_commands(cfg, out)?;

        writeln!(out, "\\pagenumbering{{alph}}")?;
        writeln!(out, "{}", preamble::THESIS_COVER)?;

        writeln!(out, "\\frontmatter{{}}")?;

        writeln!(out, "{}", preamble::THESIS_TITLE)?;

        if let Some(disclaimer) = &cfg.disclaimer {
            writeln!(out, "\\newcommand*{{\\getDisclaimer}}{{{}}}", disclaimer)?;
            writeln!(out, "{}", preamble::THESIS_DISCLAIMER)?;
        }

        writeln!(out, "\\cleardoublepage{{}}")?;

        if let Some(abstract1) = &cfg.abstract1 {
            preamble::gen_abstract(abstract1.clone(), "abstract", &Arena::new(), Thesis, cfg, out, diagnostics)?;
        }
        if let Some(abstract2) = &cfg.abstract2 {
            preamble::gen_abstract(abstract2.clone(), "abstra,t2", &Arena::new(), Thesis, cfg, out, diagnostics)?;
        }

        writeln!(out)?;
        writeln!(out, "\\microtypesetup{{protrusion=false}}")?;
        writeln!(out, "\\tableofcontents{{}}")?;
        writeln!(out, "\\microtypesetup{{protrusion=true}}")?;
        writeln!(out)?;
        writeln!(out, "\\mainmatter{{}}")?;

        Ok(())
    }

    fn gen_epilogue(&mut self, _cfg: &Config, out: &mut impl Write, _diagnostics: &Diagnostics<'a>) -> FatalResult<()> {
        writeln!(out, "\\end{{document}}")?;
        Ok(())
    }
}
