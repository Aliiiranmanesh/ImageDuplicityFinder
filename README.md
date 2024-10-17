### ImageDuplicityFinder
Find similar images in the given directory based on the content of the image regardless of size and format.
this code depends on "https://github.com/ChrisRega/image-compare" this dependency which its hybrid comparison is used.
#the similarity is calculated by :
- Splitting the image to YUV colorspace according to T.871
- Processing the Y channel with MSSIM
- Comparing U and V channels via RMS
- Recombining the differences to a nice visualization image
- RGB Score is calculated as: $\mathrm{score}=\mathrm{avg}_{x,y}\left(
  \mathrm{min}\left[\Delta \mathrm{MSSIM}(Y,x,y),\sqrt{(\Delta RMS(U,x,y))^2 + (\Delta RMS(V,x,y))^2}\right]\right)$
- RGBA can either be premultiplied with a specifiable background color using `rgba_blended_hybrid_compare`
- Otherwise, for `rgba_hybrid_compare` the $\alpha$ channel is also compared using MSSIM and taken into account.
- The average alpha of each pixel $\bar{\alpha}(x,y) = 1/2 (\alpha_1(x,y) + \alpha_2(x,y))$ is then used as a linear
  weighting factor
- RGBA Score is calculated as: $\mathrm{score}=\mathrm{avg}_{x,y}\left(1/\bar{\alpha} \cdot
  \mathrm{min}\left[\Delta \mathrm{MSSIM}(Y,x,y),\sqrt{(\Delta RMS(U,x,y))^2 + (\Delta RMS(V,x,y))^2}, \Delta \mathrm{RMS}(\alpha,x,y)\right]
  \right)$
- Edge cases RGBA: $\mathrm{score} \in (0, 1)$ and $\mathrm{score} = 1.0$ if $\bar{\alpha} = 0.0$
- This allows for a good separation of color differences and structure differences for both RGB and RGBA
- Interpretation of the diff-images:
    - RGB: Red contains structure differences, Green and Blue the color differences, the more color, the higher the diff
    - RGBA: Same as RGB but alpha contains the inverse of the alpha-diffs. If something is heavily translucent, the
      alpha was so different, that differentiating between color and structure difference would be difficult. Also,
      minimum alpha is clamped at 0.1, so you can still see all changes.
