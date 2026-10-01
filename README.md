# Website Operator HAT

Independent HATHQ HAT repository. It owns the vocabulary, context plan, exact-term reducer and procedure for `review-publication`. It contains no credentials and grants no authority. Consumers load the immutable package and invoke its declared worker interface.

The reducer accepts only canonical vocabulary IDs, rejects revision conflicts and returns `vocabulary-term-unknown` for every unrecognized value. Unknown input is never guessed or completed.

The signed package declares GitHub workflow dispatch as a service supplied by
`hat-github-operator`. The declaration describes topology only and grants no execution authority.
