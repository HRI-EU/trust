name             = 'trust'

version          = '1.0'

category         = 'Applications'

envVars          = [ ('PATH', '${PROJECT_START_PATH}/${PROJECT_NAME}/${FULL_VERSION}/bin/${MAKEFILE_PLATFORM}:${PATH}') ]

usePatchlevels   = True

patchlevel       = 0

docTool          = ''

installMatching  = [ ( 'target/release', 'trust', 'bin/${MAKEFILE_PLATFORM}' ) ]
