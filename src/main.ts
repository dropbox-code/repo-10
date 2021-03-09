import * as core from '@actions/core'
import Jenkins from 'jenkins'

export async function getJenkinsClient(
  baseUrl: string
): Promise<Jenkins.JenkinsPromisifiedAPI> {
  return Jenkins({
    baseUrl,
    promisify: true
  })
}

export async function run(): Promise<void> {
  try {
    const baseUrl: string = core.getInput('baseUrl', {
      required: true
    })

    core.info('Fetching Jenkins client')
    const jenkinsClient = await getJenkinsClient(baseUrl)

    const jobUrl: string = core.getInput('jobUrl', {
      required: true
    })
    const jobParamsString: string = core.getInput('jobParams')
    const jobParams = JSON.parse(jobParamsString)

    const buildParams = {
      delay: '0sec',
      parameters: jobParams
    }

    const getJobResponse = await jenkinsClient.job.get(jobUrl)

    if (getJobResponse.nextBuildNumber === 1) {
      core.warning(
        `Triggering Jenkins job: ${jobUrl} without params as it is the first execution`
      )

      return jenkinsClient.job.build(jobUrl)
    }

    core.info(
      `Triggering Jenkins job: ${jobUrl} with params: ${JSON.stringify(
        buildParams
      )}`
    )

    return jenkinsClient.job.build(jobUrl, buildParams)
  } catch (error) {
    core.setFailed(error.message)
  }
}

run()
