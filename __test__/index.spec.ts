import test from 'ava'

import { plus100 } from '../index'

test('async function from native code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})
