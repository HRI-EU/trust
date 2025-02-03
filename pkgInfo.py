name             = 'trust'

version          = '0.9'

category         = 'Applications'

envVars          = [ ('PATH', '${PROJECT_START_PATH}/${PROJECT_NAME}/${FULL_VERSION}/bin:${PATH}') ]

usePatchlevels   = True

patchlevel       = 0

docTool          = ''

install          = [ ( 'target/release/trust', 'bin/trust' ) ]
