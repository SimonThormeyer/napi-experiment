import test from 'ava'

import { getData, putData } from '../index'

test('async function from native code', async (t) => {
  const data = "hello napi"
  await putData(data)
  t.is(await getData(), data)
})
