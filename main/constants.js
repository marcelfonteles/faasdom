/** constants with providers, languages and tests */
const AWS = 'aws';
const AZURE = 'azure';
const AZUREWINDOWS = 'azureWindows';
const GOOGLE = 'google';
const IBM = 'ibm';

const NODE = 'node';
const PYTHON = 'python';
const GO = 'go';
const DOTNET = 'dotnet';
const RUST = 'rust';

const LATENCY = 'latency';
const FACTORS = 'factors';
const MATRIX = 'matrix';
const FILESYSTEM = 'filesystem';
const CUSTOM = 'custom';

const PROVIDERS = [AWS, AZURE, AZUREWINDOWS, GOOGLE, IBM];
const LANGUAGES = [NODE, PYTHON, GO, DOTNET, RUST];
const TESTS = [LATENCY, FACTORS, MATRIX, FILESYSTEM, CUSTOM];

module.exports = {  AWS, AZURE, AZUREWINDOWS, GOOGLE, IBM,
                    NODE, PYTHON, GO, DOTNET, RUST,
                    LATENCY, FACTORS, MATRIX, FILESYSTEM, CUSTOM,
                    PROVIDERS, LANGUAGES, TESTS }