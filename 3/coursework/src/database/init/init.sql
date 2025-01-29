-- DROP DATABASE IF EXISTS repository;
CREATE DATABASE repository;
USE repository;

-- Required info for an account
CREATE TABLE Users ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(31)  UNIQUE NOT NULL,
    email       VARCHAR(255) UNIQUE NOT NULL,
    password    VARCHAR(255) NOT NULL,

    last_used   TIMESTAMP NULL,

    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Enables multiple packages to have the same base yet different components
CREATE TABLE PackageBases ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(127) UNIQUE NOT NULL,
    description VARCHAR(510) NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL
);

-- User roles for working on packages: flagger, packager, submitter, maintainer, etc.
CREATE TABLE PackageBaseRoles ( id TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(31) UNIQUE NOT NULL,
    description VARCHAR(255) NULL
);
-- Roles that a user has for a package
CREATE TABLE PackageBaseUserRoles (
    base_id INT UNSIGNED,
    user_id INT UNSIGNED,
    role_id TINYINT UNSIGNED,

    comment VARCHAR(255) NULL,

    PRIMARY KEY (base_id, user_id, role_id), -- composite key
    FOREIGN KEY (base_id) REFERENCES PackageBases(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES PackageBaseRoles(id) ON DELETE CASCADE
);

-- Information about the actual packages
CREATE TABLE Packages ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    package_base INT UNSIGNED NOT NULL,
    name        VARCHAR(127) UNIQUE NOT NULL,
    version     VARCHAR(127) NOT NULL,
    description VARCHAR(255) NULL,
    url         VARCHAR(510) NULL,

    flagged_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,

    FOREIGN KEY (package_base) REFERENCES PackageBases (id) ON DELETE CASCADE
);

-- depends, makedepends, optdepends, etc.
CREATE TABLE DependencyTypes ( id TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(31) UNIQUE NOT NULL
);
INSERT INTO DependencyTypes (id, name) VALUES
(1, 'depends'),
(2, 'makedepends'),
(3, 'checkdepends'),
(4, 'optdepends');

-- Track which dependencies a package has
CREATE TABLE PackageDependencies ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    arch        VARCHAR(63)  NULL,
    condition   VARCHAR(255) NULL,
    description VARCHAR(127) NULL,

    package INT UNSIGNED NOT NULL,
    dependency_type TINYINT UNSIGNED NOT NULL,
    dependency_package_name VARCHAR(127) NOT NULL, -- Not an actual package, but an an alias. Allows for package substitution.

    FOREIGN KEY (package) REFERENCES Packages (id) ON DELETE CASCADE,
    FOREIGN KEY (dependency_type) REFERENCES DependencyTypes (id)
);

-- conflicts, provides, replaces, etc.
CREATE TABLE RelationTypes ( id TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(31) UNIQUE NOT NULL
);
INSERT INTO RelationTypes (id, name) VALUES
(1, 'conflicts'),
(2, 'provides'),
(3, 'replaces');

-- Track which conflicts, provides and replaces a package has
CREATE TABLE PackageRelations ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    arch        VARCHAR(63) NULL,
    condition   VARCHAR(255) NULL,

    package INT UNSIGNED NOT NULL,
    relation_type TINYINT UNSIGNED NOT NULL,
    relation_package_name VARCHAR(127) NOT NULL,

    FOREIGN KEY (package) REFERENCES Packages (id) ON DELETE CASCADE,
    FOREIGN KEY (relation_type) REFERENCES RelationTypes (id)
);

-- Public user profile
/* CREATE TABLE UserProfiles ( user_id INT UNSIGNED PRIMARY KEY,
    real_name   VARCHAR(63) NULL,
    homepage    TEXT NULL, -- bio / description / whatever
    irc_nick    VARCHAR(31) NULL,
    pgp_key     CHAR(40) NULL,
    language    VARCHAR(31) NULL, -- only for display

    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,

    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
); */

-- Settings for the User
/* CREATE TABLE UserPreferences ( user_id INT UNSIGNED PRIMARY KEY,
    inactive        BOOLEAN DEFAULT 0 NOT NULL, -- user is no longer active
    show_email      BOOLEAN DEFAULT 0 NOT NULL, -- on public profile page
    utc_timezone    TINYINT DEFAULT 0 NOT NULL, -- adjust timestamps shown
    backup_email    VARCHAR(127) NULL, -- to restore the account

    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,

    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
); */

-- Levels of access to the repository
/* CREATE TABLE AccessRoles ( id TINYINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(31) UNIQUE NOT NULL,
    description VARCHAR(255) NULL
); */
-- Roles that a user has
/* CREATE TABLE UserAccessRoles (
    user_id INT UNSIGNED,
    role_id TINYINT UNSIGNED,

    PRIMARY KEY (user_id, role_id), -- composite key
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES AccessRoles(id) ON DELETE CASCADE
); */

-- Votes
/* CREATE TABLE PackageBaseUserVotes (
    package_base INT UNSIGNED,
    user         INT UNSIGNED,
    score        TINYINT UNSIGNED DEFAULT 0 NOT NULL CHECK (score <= 10),

    comment     VARCHAR(255) NULL,
    log         TEXT NULL, -- error logs, etc.

    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,

    PRIMARY KEY (package_base, user), -- composite key
    FOREIGN KEY (package_base) REFERENCES PackageBases (id) ON DELETE CASCADE,
    FOREIGN KEY (user) REFERENCES Users (id) ON DELETE CASCADE
); */

-- Information about licenses
/* CREATE TABLE Licenses ( id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(127) UNIQUE NOT NULL,
    description TEXT NULL
); */
-- Information about licenses
/* CREATE TABLE PackageLicenses (
    package INT UNSIGNED,
    license INT UNSIGNED,

    PRIMARY KEY (package, license), -- composite key
    FOREIGN KEY (package) REFERENCES Packages (id) ON DELETE CASCADE,
    FOREIGN KEY (license) REFERENCES Licenses (id) ON DELETE CASCADE
); */
