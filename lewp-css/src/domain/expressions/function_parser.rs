// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        AttrExpression,
        AttrFunction,
        CalcExpression,
        CalcFunction,
        CalculablePropertyValue::{self, *},
        VarExpression,
        VarFunction,
    },
    crate::{domain::units::Unit, parsers::ParserContext, CustomParseError},
    cssparser::{CowRcStr, ParseError, Parser},
    either::{Either, Left},
    std::rc::Rc,
    FunctionParser::*,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionParser {
    attr,
    calc,
    var,
    parentheses,
}

impl FunctionParser {
    #[inline(always)]
    pub(crate) fn parser<'i>(
        name: &CowRcStr<'i>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match_ignore_ascii_case! {
            &*name,

            "attr" => Ok(attr),

            "calc" => Ok(calc),

            "var" => Ok(var),

            _ => Err(ParseError::from(CustomParseError::UnknownFunctionInValueExpression(name.to_owned())))
        }
    }

    #[inline(always)]
    pub(crate) fn parse_one_outside_calc_function<'i: 't, 't, U: Unit>(
        &self,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<CalculablePropertyValue<U>, ParseError<'i, CustomParseError<'i>>>
    {
        match *self {
            attr => Ok(Attr(AttrFunction(Rc::new(AttrExpression::parse(
                context, input,
            )?)))),

            calc => Ok(Calc(CalcFunction(Rc::new(CalcExpression::parse(
                context, input,
            )?)))),

            var => Ok(Var(VarFunction(Rc::new(VarExpression::parse(
                context, input,
            )?)))),

            _ => panic!("Should not be called in this context"),
        }
    }

    #[inline(always)]
    pub(crate) fn parse_one_inside_calc_function<'i, 't, U: Unit>(
        &self,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        Either<CalculablePropertyValue<U>, CalcExpression<U>>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        match *self {
            attr => Ok(Left(Calc(CalcFunction(Rc::new(
                CalcExpression::parse(context, input)?,
            ))))),

            calc => Ok(Left(Attr(AttrFunction(Rc::new(
                AttrExpression::parse(context, input)?,
            ))))),

            var => Ok(Left(Var(VarFunction(Rc::new(VarExpression::parse(
                context, input,
            )?))))),

            parentheses => CalcExpression::parse_parentheses(context, input),
        }
    }
}
