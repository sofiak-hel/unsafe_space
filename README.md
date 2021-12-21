
# Unsafe Space

[![Rust](https://github.com/sofiak-hel/unsafe_space/actions/workflows/rust.yml/badge.svg)](https://github.com/sofiak-hel/unsafe_space/actions/workflows/rust.yml)

Ever wanted to host your very own Twitter, except with a lot of excitement from
all the vulnerabilities and possibilities you'll be giving to hackers to host
it? Enter Unsafe Space, The Twitter-like server for you!

## Usage

### Download pre-compiled:
(From the [Releases](https://github.com/sofiak-hel/unsafe_space/releases)-page)
- [Windows](https://github.com/sofiak-hel/unsafe_space/releases/download/v1.0.0/unsafe_space.windows.zip)
- [Linux](https://github.com/sofiak-hel/unsafe_space/releases/download/v1.0.0/unsafe_space.linux.zip)
- [MacOS](https://github.com/sofiak-hel/unsafe_space/releases/download/v1.0.0/unsafe_space.macos.zip)

The .zip should come with a `config.toml` and a `static` folder. It is important
that both are in the working directory (same directory) when you run the
application.

### Compile and run yourself
Compiling this project yourself is fairly simple:
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. run `cargo run` in the repository. The server should now be open at 127.0.0.1:8080

## Some fun injections and XSS to try out:

- Want to log into someone else's account?
Simply log in with no username and this as the password:   
`' union select id, username from Users where username='<username>`. Just
replace `<username>` with the person's username you want to login as!
- Want to see everyone's username and password? 
  - Try pasting this into your bio `' || (select group_concat(username) || ":" || group_concat(password) from Users) where id=3;`, 
  - or alternatively, just post this message `' || (select group_concat(username) || group_concat(password) from Users) || '`
- Try simply posting `<script>alert("XSS!")</script>`

# Known vulnerabilities from the [OWASP top 10 list 2021](https://owasp.org/www-project-top-ten/)

## 1. [A01:2021 – Broken Access Control](https://owasp.org/Top10/A01_2021-Broken_Access_Control/)
### Links
- https://github.com/sofiak-hel/unsafe_space/blob/main/src/pages/mod.rs#L32

### Description
Deleting messages is done via GET-method, which makes it very easy to lure
others into deleting their messages, even without JavaScript. In addition there
is no checking of session id when deleting messages, so anyone with the message
id can delete it. CSRF is also not implemented in any way, shape, or form. This makes
Cross-Site request forgery trivial.

### How to fix?
Check Session ID when deleting messages similarly to other endpoints. Use
POST-method for deleting messages, as it is more secure, and add CSRF tokens for
all forms.

## 2. [A02:2021 – Cryptographic Failures](https://owasp.org/Top10/A02_2021-Cryptographic_Failures/)
### Links
- This applies to many parts of the code, no one line to fix it.

### Description
TLS is in no way enforced and no encryption is used anywhere in the server
software. Even in the database passwords are stored as plaintext.

### How to fix?
Check for TLS every time when reacting to requests. Also use a cryptographic
hash function when storing passwords. Actix web also supports
[rusttls](https://github.com/rustls/rustls) which could provide easy tls support
without needing a reverse proxy.

## 3. [A03:2021 - Injection](https://owasp.org/Top10/A03_2021-Injection/)  
### Links
- This applies to many parts of the code, no one line to fix it, although it can
  be fixed entirely via modifying these two files:
  - https://github.com/sofiak-hel/unsafe_space/blob/main/src/db/mod.rs
  - https://github.com/sofiak-hel/unsafe_space/blob/main/src/pages/html/timeline.html

### Description
Database injections are possible, and quite easy. XSS also is remarkably easy.

### How to fix?
Use the parameter-functionality provided by
[rusqlite](https://github.com/rusqlite/rusqlite) when executing sql queries,
instead of regular `format!`. For XSS, use the regular `{{double brackets}}` in
handlebars instead of the special non-sanitized `{{{triple brackets}}}`

## 4. [A04:2021 – Insecure Design](https://owasp.org/Top10/A04_2021-Insecure_Design/)
### Links
- No single line to point this out either, unfortunately.

### Description
This project in no shape or form uses secure design. It does have CI, but only
to build the project. It does not even have actual unit tests to speak of.

### How to fix?
Get qualified, use proper unit tests and threat modeling. Use penetration and
attack tests performed by actual professionals. Use rate-limiting.

## 5. [A05:2021 – Security Misconfiguration](https://owasp.org/Top10/A05_2021-Security_Misconfiguration/)
### Links
- https://github.com/sofiak-hel/unsafe_space/blob/main/src/db/auth.rs#L65
- https://github.com/sofiak-hel/unsafe_space/blob/main/src/db/sql/init.sql#L8

### Description
Session cookies do not have
[`Secure`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#restrict_access_to_cookies)-attribute,
which makes man-in-the-middle attacks easy to perform. There is also a default
user for `otus` that is created always by default. In the case of errors from
the SQL databases also, errors are raported directly to the user, making sql
injections trivial.

### How to fix?
Use Secure-attribute in cookies. Also go through all the pages-files and instead
of simply returning the error, return something more vague and log the error in
the server logs instead.

## 6. [~~A06:2021 – Vulnerable and Outdated Components~~](https://owasp.org/Top10/A06_2021-Vulnerable_and_Outdated_Components/)
As of writing Unsafe Space does not have this vulnerability. However, as I will
not be maintaining this project, this will one day apply.

## 7. [A07:2021 – Identification and Authentication Failures](https://owasp.org/Top10/A07_2021-Identification_and_Authentication_Failures/)
### Links
- https://github.com/sofiak-hel/unsafe_space/blob/main/src/db/mod.rs#L117
- https://github.com/sofiak-hel/unsafe_space/blob/main/src/db/mod.rs#L75

### Description
Many issues in this vulnerability apply. 
- Credential shuffling is permitted via the fact that session ID's are numerical
  and incremental. 
- Every kind of brute-force attack is allowed, as there is no encryption or
  rate-limiting involved. 
- All types of passwords are allowed. 
- Passwords are stored as plain-text in the database.
- No multi-factor authentication

### How to fix?
- Session ID's may be possible to fix simply by generating a very long random
  integer session id for each user. It will still not meet security standards,
  but it will be miles better.
- Creating middleware for actix for detecting attacks and rate-limiting may
  solve this problem.
- Get a list of most popular passwords and match incoming passwords in register
  forms with it.
- Use a hashing function, such as Argon2, for passwords before saving them to the database.
- Implement some kind of 2FA, such as [TOTP](https://en.wikipedia.org/wiki/Time-based_One-Time_Password)

## 8. [~~A08:2021 – Software and Data Integrity Failures~~](https://owasp.org/Top10/A08_2021-Software_and_Data_Integrity_Failures/)
Unsafe Space does not seem to match the criteria of this vulnerability.

## 9. [A09:2021 – Security Logging and Monitoring Failures](https://owasp.org/Top10/A09_2021-Security_Logging_and_Monitoring_Failures/)
### Links
- No single line to point this out

### Description
Even though the software does have quite adequate access logs provided by [actix
web](https://actix.rs/), simple access logs are nowhere near enough information
for detecting actual real-time attacks.

### How to fix?
A smart way to go about this would probably be to create
[middleware](https://actix.rs/docs/middleware/) for actix that detects any
irregularities or brute-force attempts and prevents and logs them.

## 10. [A10:2021-Server-Side Request Forgery](https://owasp.org/Top10/A10_2021-Server-Side_Request_Forgery_%28SSRF%29/)
### Links
- No single line for this flaw either.

### Description
Database injections are very easy, which can then be interpreted as SSRF attacks
if used to, for example, dropping tables.

### How to fix?
Simply fix database injections by using parameter-functionality from the sqlite
library as explained in flaw 3