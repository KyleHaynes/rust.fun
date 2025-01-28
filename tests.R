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
#   expression   min median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
#   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>    <dbl> <int> <dbl>   <bch:tm>
# 1 baser_date 17.0   16.9       1.78      2.01     1        3     1     16.33s
# 2 baser_char 30.1   30.3       1         4.01     1.69     3     3     29.03s
# 3 rust_date   1.11   1.18     23.3       1.01    13.1      3     1      1.25s
# 4 rust_char   1      1        28.3       1       15.9      3     1      1.03s

dt <- data.table(baser_date = baser_date, baser_char = baser_char, rust_date = rust_date, rust_char = rust_char)
View(dt)

all.equal(dt[[1]], dt[[2]])
all.equal(dt[[1]], dt[[3]])
all.equal(dt[[1]], dt[[4]])



# ---- Standardisation ----

generate_random_strings <- function(n, min_length = 5, max_length = 35) {
  chars <- c(letters, LETTERS, 0:9, rep(" ", 100))
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
#   expression   min median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
#   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>    <dbl> <int> <dbl>   <bch:tm>
# 1 base        4.48   4.46      1         3.64      NaN     3     0      8.96s
# 2 rust        1      1         4.48      1         NaN     3     0         2s


require(stringdist)
s1 <- generate_random_strings(4E5)
s2 <- generate_random_strings(4E5)

bm <- mark(
  stringdist = {
    s <- stringdist(s1, s2, method = "jw")
  },
  rust = {
    r <- compute_jaro_winkler_distance(s1, s2, T)
  }
, relative = T, min_iterations = 2, check = F)
bm


}
