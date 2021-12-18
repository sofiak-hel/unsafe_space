
# Unsafe Space

Ever wanted to host your very own Twitter, except with a lot of excitement from
all the vulnerabilities and possibilities you'll be giving to hackers to host
it? Enter Unsafe Space, The Twitter-like server for you!

## Known vulnerabilities

- Want to log into someone else's account? Simply log in with no username and
  this as the password:   
  `' union select id, username from Users where username='<username>`. Just
  replace `<username>` with the person's username you want to login as!