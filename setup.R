if(F){
    # setup package instruction (from Josiah's youtube vid)
    usethis::create_package()
    rextendr::use_extendr() # This creates the cargo.toml / bunch of dirs etc.
    rextendr::document()



# Test
d <- (sample(c(seq(as.Date('1923/01/01'), as.Date('2023/01/01'), by="day"), rep(NA, 300)), 1E6, replace = T))
dc <- as.character(d)


library(bench)
library(data.table)

bm <- mark(
  baser_date = {
    baser_date <- format((d), "%Y-%m-%d")
  },
  baser_char = {
    baser_char <- format(as.Date(dc, format = "%Y-%m-%d"), "%Y-%m-%d")
  },
  rust_date = {
    rust_date <- formatd(d, "%Y-%m-%d")
  },
  rust_char = {
    rust_char <- formatd(dc, "%Y-%m-%d")
  }

, relative = T, min_iterations = 3, check = F)
bm
# A tibble: 4 × 13
#   expression   min median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
#   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>    <dbl> <int> <dbl>   <bch:tm>
# 1 baser_date  8.91   8.89      1.73      2.01     1.73     3     5     16.17s
# 2 baser_char 15.5   15.4       1         4.01     1        3     5     27.99s
# 3 rust_date   1      1        15.4       1.00     6.16     3     2      1.82s
# 4 rust_char   1.09   1.07     14.2       1        2.83     3     1      1.98s

dt <- data.table(baser_date = baser_date, baser_char = baser_char, rust_date = rust_date, rust_char = rust_char)
View(dt)

all.equal(dt[[1]], dt[[2]])
all.equal(dt[[1]], dt[[3]])
all.equal(dt[[1]], dt[[4]])



# ---- Standardisation ----

generate_random_strings <- function(n, min_length = 5, max_length = 35) {
  chars <- c(letters, LETTERS, 0:9, rep(" ", 100))  # Characters to sample from (a-z, A-Z, 0-9)
  
  strings <- sapply(1:n, function(x) {
    len <- sample(min_length:max_length, 1)  # Random length for each string
    paste0(sample(chars, len, replace = TRUE), collapse = "")
  })
  
  return(strings)
}

# Example usage
set.seed(123)  # For reproducibility
random_strings <- generate_random_strings(1E6)

base <- function(x){
    x <- gsub("  +", " ", x, perl = T)
    x <- toupper(x)
    x <- trimws(x)
    x
}

bm <- mark(
  base = {
    basec <- base(random_strings)
  },
  rust = {
    rust <- standardise_strings(random_strings, T, T, T)
  }
, relative = T, min_iterations = 3, check = T)
bm




}