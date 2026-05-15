use ochre_interp::{error::Result, interpreter::add_source, lexer::Lexer};

fn main() -> Result<()> {
    let idx = add_source(
        "test.ocr",
        r###"
-- IMPORTS --
-- import top-level definitions (variables, enums & type aliases)
import {
  std::io,
  root::themes::{ self, gruvbox },
  conf
};

-- PRIMITIVE TYPES --
unit               = ();
decimalInt         = 42;
binaryInt          = 0b0101;
octalInt           = 0o27;
hexInt             = 0xDEADBEEF;
float              = 3.14;
bool               = false;
char               = 'c';
string             = "foo bar";
interpolatedString = "${42 |> typeOf |> toLower}erpolated ${":3" |> typeOf |> toLower}";
rawString          = r#"foo bar"#;

-- ESCAPE SEQUENCES --
newline        = '\n';
carriageReturn = '\r';
tab            = '\t';
backslash      = '\\';
null           = '\0';
dollar         = '\$';
unicode        = '\u{1f98a}';

singleQuote    = '\'';
doubleQuote    = "\"";

-- VISIBILITY --
-- accessible through the module namespace e.g. module::public
pub public = ":D";
-- not accessible
private    = ":c";

-- VARIABLE ACCESS --
-- any variable can refer to top-level definitions out of order
before = after; -- [ 1, 2, 3 ]
after = [ 1, 2, 3 ];


-- MUTATION --
-- mutation is strictly controlled and contained through the `mut` keyword
-- NOTE: `mut` can only be used within `let..in` expressions, meaning
-- variables declared within can only be mutated from the same scope or child scopes
-- and thus *never* from the top-level or even from another module
mutated = let
  x = 4;
  y = 5;
  x <- y;
in
  x; -- returns 5

-- RECORDS --
record = let
  recordA
    : { "stringKey" : Bool }
    = { "stringKey" = false };
  mut recordB
    : { key : String, "stringKey" : Bool, .. }
  -- recordB => { key = "value", "stringKey" = false, nested = { key = 12 } }
    = { key = "value", ..record, nested.key = 12 };
  -- recordB => { key = "value", "stringKey" = true }
  recordB."stringKey" <- true;
  -- recordB => { key = "value", "stringKey" = true, newKey = ":D" }
  recordB.newKey <- ":D";
  -- NOTE: we can only do this if the type of `record` allows this
  -- so we must either explicitly declare that `record` has a field
  -- called `newKey` of type `String` OR that it has miscellaneous fields.
  -- if `record` didn't have an explicit type, it would be inferred BEFORE we
  -- update its value, and thus would have no knowledge of `newKey`, resulting in
  -- a type error
in
  recordB; -- { key = "value", "stringKey" = true, newKey = ":D" }

-- ARRAYS --
myArray       = [ ":3", ":o", ":D" ];
element       = myArray[0];
destructuring = [ ..myArray, ":c" ];

-- TUPLES --
tuple       = (5, 1);
tupleAccess = tuple.0; -- 5

-- tuple types can also have named fields
type Vec2     = (x: Float, y: Float);
pointA : Vec2 = (0.5, 1.5);
pointB : Vec2 = (2.0, 0.0);
dir    : Vec2 = (pointB.y - pointA.y, pointB.x - pointA.x);

-- CLOSURES --
-- closure arguments MUST specify their types
closure            = \(a : Int) -> \(b : Int) -> a + b;
call               = closure 1 2;

-- TYPE INFORMATION --
inferredType       = ":3";
explicitType : Int = 7;

-- type aliases and newtypes are both ways of declaring a wrapper
-- around an underlying type, with one major difference:
type TypeAlias   = [(Float, Int)];

type UnitType    = ();
type AnyType     = Any;
type CharType    = Char;
type IntType     = Int;
type FloatType   = Float;
type BoolType    = Bool;
type StrType     = String;
type TupleType   = (Int, Bool);
type ArrayType   = [Char];
type RecordType  = { key : String, optionalKey? : Float, "stringKey" : Bool, .. };
type ClosureType = Int -> Int -> Int;

-- OPERATORS --
-- * / + -   arithmetic
-- ! -       unary
-- > >= <= < comparison
-- ++        concatenation
-- //        update
-- |>        piping
-- >>        chaining
-- <-        assignment

-- PIPING
-- pipe the output of a closure into the input of another closure
-- it essentially reverses the order of call expressions
-- which is useful for chaining multiple function calls to avoid
-- deeply nested parentheses
fn     = \(str: String) println str;
-- this expression:
piped  = ":3" |> fn;
-- is equivalent to this expression:
called = fn ":3";

-- CHAINING --
-- the chain operator allows for arbitrary expressions to be
-- evaluated one after another
-- the final expression in the chain is the return value of the whole chain
-- and the return values of all other expressions in the chain are discarded
exprA = println "exprA";
exprB = println "exprB";
exprC = println "exprC";
chain = exprA >> exprB >> exprC; -- returns the result of evaluating `exprC`

-- KEYWORDS --
ifThenElse     = if isOdd 1 then "odd" else "even";

andOr          = true and (false or true);

-- declare locally scoped variables within an expression
letIn          = let x = 4; in x;

inherited =
  let
    foo = ":D";
    bar = ":o";
  in
    { inherit foo bar }; -- equivalent to { foo = foo, bar = bar }

-- COMMENTS --
-- line comment

{-
  block comment
-}

@-
  # This is a doc comment

  ## Test

  - Doc comments are used to provide documentation for variables/types
  - Their content should follow the Markdown spec.
-@


-- MATCH --
-- match arms can match on literal expressions or on expression patterns
-- which bind variables to parts of the expression (or the whole expression)
-- more specific patterns should be placed before less specific ones as match
-- arms will match on the first pattern that fits
enum Value = Boolean Bool | Number Float | String String | Null;
stringify : Value -> String
  = \(value: Value) -> match myType;
    Value::Boolean bool => "${bool}",
    Value::Number 69 => "${num} (nice)",
    Value::Number num => "${num}",
    Value::String str => "${str}",
    _ => "<null>";

-- MAIN --
-- the root module must have a variable called `main` which is a closure
-- this is used as the entry point to your program
main : [String] -> ()
  =  \(args: [String]) -> let
      name = args[0];
    in
      println "Hello, ${name}!";
            
        "###,
        None,
    )?;

    let mut lexer = Lexer::new(idx)?;
    let res = lexer.lex_tokens();

    println!("{}", lexer.tokens());

    res.unwrap();

    Ok(())
}
