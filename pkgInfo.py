name             = 'trust'

version          = '1.0'

category         = 'Applications'

envVars          = [ ('PATH', '${PROJECT_START_PATH}/${PROJECT_NAME}/${FULL_VERSION}/bin:${PATH}') ]

usePatchlevels   = True

patchlevel       = 1

docTool          = ''

installMatching  = [ ( 'target/release', 'trust', 'bin/${MAKEFILE_PLATFORM}' ) ]
