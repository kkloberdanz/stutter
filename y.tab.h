/* A Bison parser, made by GNU Bison 3.0.4.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

#ifndef YY_YY_Y_TAB_H_INCLUDED
# define YY_YY_Y_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token type.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    ENDFILE = 258,
    ERROR = 259,
    IF = 260,
    THEN = 261,
    ELSE = 262,
    PRINT = 263,
    ID = 264,
    NUMBER = 265,
    ASSIGN = 266,
    EQ = 267,
    NE = 268,
    LT = 269,
    GE = 270,
    LE = 271,
    PLUS = 272,
    MINUS = 273,
    TIMES = 274,
    OVER = 275,
    EXPONENT = 276,
    LPAREN = 277,
    RPAREN = 278,
    SEMI = 279
  };
#endif
/* Tokens.  */
#define ENDFILE 258
#define ERROR 259
#define IF 260
#define THEN 261
#define ELSE 262
#define PRINT 263
#define ID 264
#define NUMBER 265
#define ASSIGN 266
#define EQ 267
#define NE 268
#define LT 269
#define GE 270
#define LE 271
#define PLUS 272
#define MINUS 273
#define TIMES 274
#define OVER 275
#define EXPONENT 276
#define LPAREN 277
#define RPAREN 278
#define SEMI 279

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
typedef int YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;

int yyparse (void);

#endif /* !YY_YY_Y_TAB_H_INCLUDED  */
