pipeline {
    agent any

    environment {
        DOCKER_REGISTRY = credentials('docker-registry')
        VERSION = sh(script: 'git describe --tags --always', returnStdout: true).trim()
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Backend Tests') {
            steps {
                dir('backend') {
                    sh 'cargo test'
                    sh 'cargo clippy -- -D warnings'
                }
            }
        }

        stage('Frontend Tests') {
            steps {
                dir('frontend') {
                    sh 'npm install'
                    sh 'npx vitest'
                    sh 'npm run lint'
                }
            }
        }

        stage('Build & Push') {
            steps {
                script {
                    docker.withRegistry("http://${DOCKER_REGISTRY}") {
                        def backend_build = docker.build("${DOCKER_REGISTRY}/backend:${VERSION}", "./backend")
                        def frontend_build = docker.build("${DOCKER_REGISTRY}/frontend:${VERSION}", "./frontend")

                        backend_build.push()
                        frontend_build.push()
                    }
                }
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            echo 'Pipeline succeeded!'
        }
        failure {
            echo 'Pipeline failed!'
        }
    }
}
