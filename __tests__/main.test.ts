jest.mock('jenkins')
import Jenkins from 'jenkins'

import * as main from '../src/main'

describe('run tests', () => {
  beforeEach(() => {
    process.env.INPUT_JOBPARAMS =
      '{"CLEAN_WS":false,"CT_JENKINS_DEBUG":false,"FORCE_BUILD":false}'
    process.env.INPUT_JOBURL =
      'https://jenkins.foo.com/job/foo/job/bar/job/PR-12345'
    process.env.INPUT_BASEURL = 'https://jenkins.foo.com'
  })

  afterEach(() => {
    delete process.env.INPUT_JOBPARAMS
    delete process.env.INPUT_JOBURL
    delete process.env.INPUT_BASEURL
    jest.restoreAllMocks()
  })

  test('should create jenkins client', async () => {
    await main.run()
    expect(Jenkins).toHaveBeenNthCalledWith(1, {
      baseUrl: process.env.INPUT_BASEURL,
      promisify: true
    })
  })
})
