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
                    sh 'npm run test'
                    sh 'npm run lint'
                }
            }
        }

        stage('Build & Push') {
            steps {
                script {
                    docker.build("${DOCKER_REGISTRY}/backend:${VERSION}", "./backend")
                    docker.push("${DOCKER_REGISTRY}/backend:${VERSION}")

                    docker.build("${DOCKER_REGISTRY}/frontend:${VERSION}", "./frontend")
                    docker.push("${DOCKER_REGISTRY}/frontend:${VERSION}")
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
