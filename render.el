(save-excursion
  ;; clear existing index.md
  (switch-to-buffer "index.md")
  (kill-region (point-min) (point-max))

  (insert "### Tutorial examples\n\n")

  (insert "- [Directions for installing Rust][d], should you choose to do so.\n")
  (insert "- [API docs][api] for Rust.\n")

  (let ((examples '(0 10 20 30 40 50 60 70 80)))

    ;; Generate the html files.
    (dolist (n examples)
      (switch-to-buffer (format "example%03d.rs" n))
      (htmlfontify-buffer)
      (switch-to-buffer (format "example%03d.rs.html" n))
      (beginning-of-buffer)
      (replace-string "font-size: 0pt;" "font-size: 12pt;")
      (save-buffer)
      (kill-buffer))

    ;; Insert the index.md links
    (switch-to-buffer "index.md")
    (dolist (n examples)
      (insert (format "- Example %03d: [html][html%d], [play][play%d]\n"
                      n n n)))

    ;; Generate the html links
    (insert "\n")
    (dolist (n examples)
      (insert (format "[html%d]: example%03d.rs.html\n" n n)))

    ;; Generate the actual links
    (dolist (n examples)
      (switch-to-buffer (format "example%03d.rs" n))
      (let* ((text (buffer-string))
             (text (replace-regexp-in-string "/\\*[^/]*\\*/" "" text)))
        (switch-to-buffer "index.md")
        (insert (format "[play%d]: http://play.rust-lang.org/?code=%s\n"
                        n (url-hexify-string text)))))
    )

  (switch-to-buffer "index.md")
  (insert "[api]: http://doc.rust-lang.org/std/index.html\n")
  (insert "[d]: http://doc.rust-lang.org/guide.html#installing-rust\n")
  (save-buffer)
  )

