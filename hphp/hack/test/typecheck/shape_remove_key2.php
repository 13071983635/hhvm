<?hh //strict

// Removing field from intersection of shapes

type s = shape(
  'x' => string,
  'y' => int,
);
type t = shape(
  'x' => bool,
  'y' => num,
);

function test(bool $b, s $s, t $t): void {
  if ($b) {
    $st = $s;
  } else {
    $st = $t;
  }

  Shapes::idx($st, 'x'); // no error
  Shapes::removeKey($st, 'x');
  Shapes::idx($st, 'y'); // no error
  Shapes::idx($st, 'x'); // error
}
