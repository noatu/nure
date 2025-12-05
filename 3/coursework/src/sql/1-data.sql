USE repository;

-- Insert Users
INSERT INTO Users (name, email, password, last_used) VALUES
('alice', 'alice@example.com', 'password123', NOW()),
('bob', 'bob@example.com', 'securepass', NOW()),
('charlie', 'charlie@example.com', 'charliepwd', NOW()),
('dave', 'dave@example.com', 'davepass', NOW()),
('eve', 'eve@example.com', 'evepwd', NOW()),
('frank', 'frank@example.com', 'frankpass', NOW()),
('grace', 'grace@example.com', 'gracepwd', NOW()),
('heidi', 'heidi@example.com', 'heidipwd', NOW()),
('ivan', 'ivan@example.com', 'ivanpass', NOW()),
('judy', 'judy@example.com', 'judypass', NOW()),
('mallory', 'mallory@example.com', 'mallorypwd', NOW()),
('oscar', 'oscar@example.com', 'oscarpass', NOW()),
('peggy', 'peggy@example.com', 'peggypwd', NOW()),
('trent', 'trent@example.com', 'trentpass', NOW()),
('victor', 'victor@example.com', 'victorpwd', NOW()),
('test', 'test@example.com', '$argon2id$v=19$m=19456,t=2,p=1$4POPE1A9A3evhePMYON7FQ$78A3OImH975IwugMikARwoEHkPixKJxJwPXgsCnQdY0', NOW());


-- Insert PackageBases
INSERT INTO PackageBases (name, description) VALUES
('libcore', 'Core system libraries'),
('webframework', 'A modern web framework'),
('dataproc', 'Data processing toolkit'),
('authmodule', 'Authentication and authorization module'),
('networkstack', 'Networking utilities and stack'),
('uikit', 'UI Kit for building interfaces'),
('cryptoengine', 'Cryptographic library'),
('dbconnector', 'Database connectivity drivers'),
('imageproc', 'Image processing library'),
('audiokit', 'Audio toolkit'),
('videokit', 'Video processing toolkit'),
('mlcore', 'Machine Learning core library'),
('analyticspro', 'Advanced analytics toolkit'),
('monitoragent', 'System monitoring agent'),
('filesystem', 'Filesystem utilities');

-- Assign Roles to Users for PackageBases
INSERT INTO PackageBaseUserRoles (base, user, role, comment) VALUES
(1, 1, 1, 'Original submitter'),
(1, 2, 2, 'Packager for latest release'),
(2, 3, 3, 'Maintains stability'),
(2, 4, 4, 'Flags issues'),
(3, 5, 1, 'Initial submission'),
(3, 6, 3, 'Lead maintainer'),
(4, 7, 2, 'Core packager'),
(5, 8, 1, 'Submitted new version'),
(6, 9, 4, 'Flagged for performance issues'),
(7, 10, 3, 'Maintainer for security fixes'),
(8, 11, 2, 'Driver package manager'),
(9, 12, 1, 'Original contributor'),
(10, 13, 3, 'Maintains core features'),
(11, 14, 4, 'Reported critical bug'),
(12, 15, 2, 'Optimized build process');

-- Insert Packages
INSERT INTO Packages (base, name, version, description, url) VALUES
(1, 'libcore-utils', '1.0.0', 'Utilities for libcore', 'http://example.com/libcore-utils'),
(1, 'libcore-extended', '1.1.0', 'Extended functionalities', 'http://example.com/libcore-extended'),
(2, 'webframework-api', '2.0.0', 'REST API module', 'http://example.com/webframework-api'),
(2, 'webframework-cli', '2.1.0', 'Command-line tools', 'http://example.com/webframework-cli'),
(3, 'dataproc-engine', '3.0.1', 'Data processing engine', 'http://example.com/dataproc-engine'),
(4, 'authmodule-oauth', '4.2.0', 'OAuth module', 'http://example.com/authmodule-oauth'),
(5, 'networkstack-core', '5.5.0', 'Core network stack', 'http://example.com/networkstack-core'),
(6, 'uikit-designer', '6.0.3', 'UI designer toolkit', 'http://example.com/uikit-designer'),
(7, 'cryptoengine-hash', '7.1.1', 'Hash algorithms', 'http://example.com/cryptoengine-hash'),
(8, 'dbconnector-mysql', '8.0.0', 'MySQL connector', 'http://example.com/dbconnector-mysql'),
(9, 'imageproc-filters', '9.3.0', 'Image filters library', 'http://example.com/imageproc-filters'),
(10, 'audiokit-mixer', '10.2.1', 'Audio mixing toolkit', 'http://example.com/audiokit-mixer'),
(11, 'videokit-stream', '11.4.0', 'Video streaming tools', 'http://example.com/videokit-stream'),
(12, 'mlcore-algo', '12.0.2', 'ML algorithms', 'http://example.com/mlcore-algo'),
(13, 'analyticspro-dashboard', '13.5.1', 'Analytics dashboard', 'http://example.com/analyticspro-dashboard');

-- Insert PackageDependencies
INSERT INTO PackageDependencies (arch, requirement, description, package, dependency_type, dependency_package_name) VALUES
('x86_64', '>=1.0.0', 'Core dependency', 3, 1, 'libcore-utils'),
('x86_64', '>=2.0.0', 'Required for API', 4, 2, 'webframework-api'),
('arm64', '>=3.0.1', 'Optional analytics', 5, 4, 'analyticspro-dashboard'),
('x86_64', '>=5.5.0', 'Network stack dependency', 6, 1, 'networkstack-core'),
('x86_64', '>=4.2.0', 'Authentication module', 7, 1, 'authmodule-oauth'),
('x86_64', NULL, 'Database driver', 8, 1, 'dbconnector-mysql'),
('arm64', NULL, 'Machine learning algorithms', 9, 3, 'mlcore-algo'),
('x86_64', '>=6.0.3', 'UI designer toolkit', 10, 1, 'uikit-designer'),
('x86_64', NULL, 'Audio toolkit dependency', 11, 2, 'audiokit-mixer'),
('x86_64', '>=7.1.1', 'Hash functions', 12, 1, 'cryptoengine-hash'),
('arm64', NULL, 'Video streaming tools', 13, 4, 'videokit-stream'),
('x86_64', '>=9.3.0', 'Image filters', 14, 1, 'imageproc-filters'),
('x86_64', NULL, 'System monitoring agent', 15, 2, 'monitoragent');

-- Insert PackageRelations
INSERT INTO PackageRelations (arch, requirement, package, relation_type, relation_package_name) VALUES
('x86_64', '>=1.0.0', 3, 1, 'legacy-web-api'), -- conflicts
('x86_64', NULL, 4, 2, 'web-cli-tools'),       -- provides
('arm64', NULL, 5, 3, 'old-dataproc'),         -- replaces
('x86_64', '>=5.0.0', 6, 1, 'net-tools-legacy'),
('x86_64', NULL, 7, 2, 'crypto-lib'),
('x86_64', '>=4.0.0', 8, 3, 'db-driver-old'),
('arm64', NULL, 9, 1, 'imgproc-v1'),
('x86_64', NULL, 10, 2, 'audio-tools'),
('x86_64', '>=7.0.0', 11, 3, 'video-kit-old'),
('x86_64', NULL, 12, 1, 'ml-core-legacy'),
('x86_64', '>=6.0.0', 13, 2, 'analytics-pro-tools'),
('x86_64', NULL, 14, 3, 'monitor-agent-v1'),
('x86_64', '>=9.0.0', 15, 1, 'filesystem-old');
