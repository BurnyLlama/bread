# API definition for Bread

## Introduction

For the simplicity of the project, the API won't be completely separate from the web application. That is to make it simpler to implement. In a web application that would also get a phone application or other client applications, third party or not, should use a system where the web application is completely separate from and independant of the API.

Routes marked with 🔐 are protected and require a token to be accessed.

## App

The web application is located on `/app`. All of these requests are `GET`-requests.

### `/app`

Landing page. Shows an option to register an account or log in to an existing account.

### `/app/login`

The log in page.

### `/app/register`

The register page.

### `/app/create-post` 🔐

Presents the user with the option to create a new post. The user can only create two posts per day.

### `/app/explore` 🔐

Presents the user with a list of all the user they follow as well as the latest post from each user.

### `/app/random` 🔐

Presents the user with a random post. The user can choose to follow the post's author or choose to see another post. The user can also choose to see another post, but can only see a maximum of ten posts per day.

### `/app/profile` 🔐

Shows the profile of the user as well as some settings that the user can change.

## API

### POST: `/api/auth/register`

**Body:**

```rust
username: String // Name for the user to register.
password: String // Password for the user.
```

Creates a new user of name `username` and password `password`, so long as a user with that name doesn't already exists.

### POST: `/api/auth/login`

**Body:**

```rust
username: String // Name of the user
password: String // Password of the user
```

Logs in the user with the given username and password, so long as that user exists and has said password. A successful login will result in giving the user a token so it can access the API.

### POST: `/api/auth/change-password` 🔐

**Body:**

```rust
old_password: String // Current/old password for the user.
new_password: String // The password the user wants to change too.
```

This lets a user change their password so long as they know their previous password.

### POST: `/api/create-post` 🔐

**Body:**

```rust
// Content for the post, can be None, or Some<String>.
content: Option<String>
// An optional image for the post, can be None or Some<String>.
image: Option<Image>
```

This lets a logged in user create post. The user can only create two posts per day. The user can have a text, image, or both, in their post.

### POST: `/api/settings` 🔐

**Body:**

```rust
username: String
profile_color: ProfileColor
prefers_darkmode: bool
```

This lets a user change their preferences, which includes username, profile color, and whether or not they want dark mode.

### POST: `/api/follow/<userid: ObjectId>` 🔐

This lets a user follow another user.

### POST: `/api/unfollow/<userid: ObjectId>` 🔐

This lets a user unfollow another user.
