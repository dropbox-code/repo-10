import * as core from '@actions/core'
import Jenkins from 'jenkins'

async function getJenkinsClient(
  baseUrl: string
): Promise<Jenkins.JenkinsPromisifiedAPI> {
  return Jenkins({
    baseUrl,
    promisify: true
  })
}

async function run(): Promise<void> {
  try {
    const baseUrl: string = core.getInput('baseUrl')

    core.info('Fetching Jenkins client')
    const jenkinsClient = await getJenkinsClient(baseUrl)

    const jobUrl: string = core.getInput('jobUrl')
    const jobParamsString: string = core.getInput('jobParams')

    const jobParams = JSON.parse(jobParamsString)

    core.info(
      `Triggering Jenkins job: ${jobUrl} with params: ${jobParamsString}`
    )
    await jenkinsClient.job.build(jobUrl, {
      delay: '0sec',
      parameters: jobParams
    })
    core.info(`Triggered Jenkins job: ${jobUrl}`)
  } catch (error) {
    core.setFailed(error.message)
  }
}

run()
