if(F){
    # setup package instruction (from Josiah's youtube vid)
    usethis::create_package()
    rextendr::use_extendr() # This creates the cargo.toml / bunch of dirs etc.
    rextendr::document()



# Test
d <- (sample(c(seq(as.Date('1923/01/01'), as.Date('2023/01/01'), by="day"), rep(NA, 300)), 1E6, replace = T))
dc <- as.character(d)


library(bench)

bm <- mark(
  baser_date = {
    baser_date <- format((d), "%d %D %Y %m xxxx")
  },
  baser_char = {
    baser_char <- format(as.Date(dc, format = "%Y-%d-%m"), "%d %D %Y %m xxxx")
  },
  rust_date = {
    rust_date <- formatd(d, "%d %D %Y %m xxxx")
  },
  rust_char = {
    rust_char <- formatd(dc, "%d %D %Y %m xxxx")
  }

, relative = T, min_iterations = 3, check = F)
bm




}