### Tutorial examples

- Example 000: [html][html0], [play][play0]
- Example 010: [html][html10], [play][play10]
- Example 020: [html][html20], [play][play20]
- Example 025: [html][html25], [play][play25]
- Example 030: [html][html30], [play][play30]
- Example 040: [html][html40], [play][play40]
- Example 050: [html][html50], [play][play50]
- Example 060: [html][html60], [play][play60]
- Example 070: [html][html70], [play][play70]
- Example 080: [html][html80], [play][play80]
- Example 090: [html][html90], [play][play90]
- Example 100: [html][html100], [play][play100]
- Example 110: [html][html110], [play][play110]
[html0]: example000.rs.html
[html10]: example010.rs.html
[html20]: example020.rs.html
[html25]: example025.rs.html
[html30]: example030.rs.html
[html40]: example040.rs.html
[html50]: example050.rs.html
[html60]: example060.rs.html
[html70]: example070.rs.html
[html80]: example080.rs.html
[html90]: example090.rs.html
[html100]: example100.rs.html
[html110]: example110.rs.html
[play0]: http://play.rust-lang.org/?code=
[play10]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Rust%20basics.%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20vec%3A%20Vec%3Cint%3E%20%3D%20Vec%3A%3Anew%28%29%3B%20%20%20%20%20%20%20%0A%0A%20%20%20%20%0A%0A%20%20%20%20vec.push%2822%29%3B%0A%20%20%20%20vec.push%2844%29%3B%0A%20%20%20%20vec.push%2866%29%3B%0A%0A%20%20%20%20println%21%28%22Vector%20has%20length%20%60%7B%7D%60%20and%20contents%20%60%7B%7D%60%22%2C%20vec.len%28%29%2C%20vec%29%3B%20%0A%0A%20%20%20%20let%20string%20%3D%20format%21%28%22Vector%20has%20length%20%60%7B%7D%60%20and%20contents%20%60%7B%7D%60%22%2C%20vec.len%28%29%2C%20vec%29%3B%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20string%29%3B%0A%0A%7D%20%2F%2F%20%3C--%20Here%2C%20%60vec%60%20goes%20out%20of%20scope%2C%20destructor%20will%20run%20and%20it%20will%20be%20freed.%0A%0A%2F%2F%20Exercise%20%231%3A%20Remove%20the%20type%20annotation.%20What%20happens%3F%20How%20can%20we%20fix%20it%3F%0A
[play20]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Ownership.%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20vec%20%3D%20vec%21%5B22%2C%2044%2C%2066%5D%3B%20%0A%0A%20%20%20%20let%20sum%20%3D%20sum%28vec%29%3B%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20println%21%28%22The%20sum%20of%20the%20vector%20is%20%60%7B%7D%60%22%2C%20sum%29%3B%0A%7D%0A%0A%2F%2F%20Function%20declarations%3A%0A%0Afn%20sum%28v%3A%20Vec%3Cint%3E%29%20-%3E%20int%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20let%20%28mut%20i%2C%20c%2C%20mut%20sum%29%20%3D%20%280%2C%20v.len%28%29%2C%200%29%3B%20%0A%0A%20%20%20%20while%20i%20%3C%20c%20%7B%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20sum%20%2B%3D%20v%5Bi%5D%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20sum%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%7D%0A%0A%2F%2F%20Exercise%20%231%3A%20Find%20and%20fix%20the%20bug.%0A%0A%2F%2F%20Exercise%20%232%3A%20Modify%20the%20%60main%60%20function%20to%20print%20both%20the%20vector%0A%2F%2F%20contents%20and%20its%20sum%20%28e.g.%2C%20%22The%20sum%20of%20%60%5B22%2C%2044%2C%2066%5D%60%20is%0A%2F%2F%20%60132%60%22%29.%20Why%20doesn%27t%20it%20compile%3F%20How%20can%20we%20modify%20%60sum%60%20to%20make%0A%2F%2F%20this%20work%3F%0A%0A%2F%2F%20Exercise%20%233%3A%20Modify%20%60sum%60%20to%20compute%20the%20prefix%20sum%20instead%20and%0A%2F%2F%20print%20the%20result.%0A
[play25]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Threading%20and%20messaging.%0A%0Ause%20std%3A%3Acomm%3B%0Ause%20std%3A%3Atask%3B%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20%28tx%2C%20rx%29%20%3D%20comm%3A%3Achannel%28%29%3B%20%0A%0A%20%20%20%20task%3A%3Aspawn%28proc%28%29%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20let%20mut%20factorials%20%3D%20Vec%3A%3Anew%28%29%3B%0A%20%20%20%20%20%20%20%20let%20mut%20i%20%3D%200%3B%0A%20%20%20%20%20%20%20%20loop%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20let%20f%20%3D%20factorial%28i%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20if%20f%20%3E%20128%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20break%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%0A%20%20%20%20%20%20%20%20%20%20%20%20factorials.push%28f%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20i%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20tx.send%28factorials%29%3B%0A%20%20%20%20%7D%29%3B%0A%0A%20%20%20%20println%21%28%22...%20do%20something%20here%20...%22%29%3B%0A%0A%20%20%20%20let%20f%20%3D%20rx.recv_opt%28%29.unwrap%28%29%3B%0A%0A%20%20%20%20println%21%28%22factorials%20up%20to%20128%20are%20%60%7B%7D%60%22%2C%20f%29%3B%0A%7D%0A%0Afn%20factorial%28n%3A%20uint%29%20-%3E%20uint%20%7B%0A%20%20%20%20if%20n%20%3D%3D%200%20%7B%201%20%7D%20else%20%7B%20n%20%2A%20factorial%28n%20-%201%29%20%7D%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Try%20to%20modify%0A
[play30]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Borrowing.%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20vec%20%3D%20vec%21%5B22%2C%2044%2C%2066%5D%3B%0A%0A%20%20%20%20let%20sum%20%3D%20sum%28%26vec%29%3B%20%20%20%20%20%20%20%0A%0A%20%20%20%20println%21%28%22The%20sum%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%20vec%2C%20sum%29%3B%0A%7D%0A%0Afn%20sum%28v%3A%20%26Vec%3Cint%3E%29%20-%3E%20int%20%7B%20%20%20%20%0A%0A%20%20%20%20let%20%28mut%20i%2C%20c%2C%20mut%20sum%29%20%3D%20%280%2C%20v.len%28%29%2C%200%29%3B%0A%0A%20%20%20%20while%20i%20%3C%20c%20%7B%0A%20%20%20%20%20%20%20%20sum%20%2B%3D%20v%5Bi%5D%3B%0A%20%20%20%20%20%20%20%20i%20%2B%3D%201%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20sum%0A%7D%0A%0A%2F%2F%20Walthrough%201.%20Convert%20to%20use%20slices.%0A%0A%2F%2F%20Exercise%202.%20Write%20a%20binary%20search%20function.%0A
[play40]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Returning%20references%20and%20borrow%20scopes.%0A%0A%23%21%5Bfeature%28slicing_syntax%29%5D%20%20%20%20%20%20%20%20%20%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20vec%20%3D%20vec%21%5B1%2C%202%2C%203%2C%204%2C%205%2C%206%2C%207%2C%208%2C%209%2C%2010%5D%3B%0A%20%20%20%20let%20%28left%2C%20right%29%20%3D%20split_at%28vec%5B%5D%2C%205%29%3B%0A%20%20%20%20println%21%28%22%60%7B%7D%60%20split%20at%205%20yields%20%60%7B%7D%60%20and%20%60%7B%7D%60%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20vec%2C%20left%2C%20right%29%3B%0A%7D%0A%0Afn%20split_at%3C%27a%3E%28slice%3A%20%26%27a%20%5Bint%5D%2C%20mid%3A%20uint%29%20-%3E%20%28%26%27a%20%5Bint%5D%2C%20%26%27a%20%5Bint%5D%29%20%7B%20%0A%0A%20%20%20%20%28slice%5B..mid%5D%2C%20slice%5Bmid..%5D%29%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Try%20inserting%20various%20calls%20to%20%60vec.push%28%29%60%20in%0A%2F%2F%20%60main%28%29%60.%20What%20happens%3F%20Does%20it%20make%20a%20difference%20where%20you%20insert%0A%2F%2F%20the%20call%3F%20Discuss.%0A%0A%2F%2F%20Exercise%202.%20What%20heppens%20if%20you%20take%20out%20all%20the%20references%20%60%27a%60%3F%0A
[play50]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Mutable%20borrowing.%0A%0Ause%20std%3A%3Amem%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2A%0A%20%20%20%20%20%20%20%20%20~~~%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2A%0A%20%20%20%20%20%20%20%20%20%20%7C%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2A%0A%20%20%20Import%20mem%20module%20into%20scope.%20%20%20%20%20%20%2A%0A%20%20%20http%3A%2F%2Fdoc.rust-lang.org%2Fstd%2Fmem%2F%20%20%2A%2F%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20vec%20%3D%20vec%21%5B22%2C%2044%2C%2066%5D%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20let%20sum%20%3D%20prefix_sum%28%26mut%20vec%29%3B%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20println%21%28%22The%20prefix%20sum%20is%20%60%7B%7D%60%2C%20%60%7B%7D%60%22%2C%20vec%2C%20sum%29%3B%0A%7D%0A%0Afn%20prefix_sum%28v%3A%20%26mut%20Vec%3Cint%3E%29%20-%3E%20int%20%7B%20%0A%0A%20%20%20%20let%20%28mut%20i%2C%20c%2C%20mut%20sum%29%20%3D%20%280%2C%20v.len%28%29%2C%200%29%3B%0A%0A%20%20%20%20while%20i%20%3C%20c%20%7B%0A%20%20%20%20%20%20%20%20let%20value%20%3D%20mem%3A%3Areplace%28%26mut%20v%5Bi%5D%2C%20sum%29%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20sum%20%2B%3D%20value%3B%0A%20%20%20%20%20%20%20%20i%20%2B%3D%201%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20sum%0A%7D%0A%0A%2F%2F%20Exercise%201%3A%20Write%20quicksort.%20What%20difficulty%20do%20you%20encounter%3F%20What%20is%20the%20reason%20for%0A%2F%2F%20this%3F%20How%20can%20it%20be%20overcome%3F%0A%2F%2F%0A%2F%2F%20Hint%20i.%20Use%20mutable%20slices%20%28%60%26mut%20%5Bint%5D%60%2C%20%60vec%5Bmut%5D%60%29.%0A%2F%2F%0A%2F%2F%20Hint%20ii.%20Look%20at%20the%20method%20%60split_at_mut%60%3A%0A%2F%2F%20%20%20%20%20%20%20%20%20%20http%3A%2F%2Fdoc.rust-lang.org%2Fstd%2Fslice%2Ftrait.MutableSlice.html%23tymethod.split_at_mut%0A
[play60]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Traits%20and%20generic%20programming.%0A%0Atrait%20Numeric%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20fn%20zero%28%29%20-%3E%20Self%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20fn%20add%28%26self%2C%20other%3A%20%26Self%29%20-%3E%20Self%3B%20%20%20%0A%7D%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20ints%3A%20Vec%3Cint%3E%20%3D%20vec%21%5B22%2C%2044%2C%2066%5D%3B%0A%20%20%20%20println%21%28%22Sum%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%20ints%2C%20sum%28%26ints%29%29%3B%0A%0A%20%20%20%20let%20f64s%3A%20Vec%3Cf64%3E%20%3D%20vec%21%5B0.5%2C%201.5%2C%202.5%5D%3B%0A%20%20%20%20println%21%28%22Sum%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%20ints%2C%20sum%28%26f64s%29%29%3B%0A%7D%0A%0Afn%20sum%3CN%3E%28vec%3A%20%26Vec%3CN%3E%29%20-%3E%20N%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20where%20N%20%3A%20Numeric%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%7B%0A%20%20%20%20let%20mut%20sum%3A%20N%20%3D%20Numeric%3A%3Azero%28%29%3B%20%20%20%20%20%20%0A%0A%20%20%20%20for%20elem%20in%20vec.iter%28%29%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20let%20intermediate%20%3D%20sum.add%28elem%29%3B%20%20%0A%0A%20%20%20%20%20%20%20%20sum%20%3D%20intermediate%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20sum%0A%7D%0A%0Aimpl%20Numeric%20for%20int%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20fn%20zero%28%29%20-%3E%20int%20%7B%200%20%7D%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20fn%20add%28%26self%2C%20other%3A%20%26int%29%20-%3E%20int%20%7B%0A%20%20%20%20%20%20%20%20%2Aself%20%2B%20%2Aother%0A%20%20%20%20%7D%0A%7D%0A%0Aimpl%20Numeric%20for%20f64%20%7B%0A%20%20%20%20fn%20zero%28%29%20-%3E%20f64%20%7B%200.0%20%7D%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20fn%20add%28%26self%2C%20other%3A%20%26f64%29%20-%3E%20f64%20%7B%0A%20%20%20%20%20%20%20%20%2Aself%20%2B%20%2Aother%0A%20%20%20%20%7D%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Add%20another%20impl%20for%20the%20type%20%60u32%60.%0A%2F%2F%20Check%20that%20it%20works.%0A%0A%2F%2F%20Exercise%202.%20Why%20did%20we%20use%20an%20intermediate%20variable%20to%20store%20the%0A%2F%2F%20result%20of%20adding%20each%20element%20and%20the%20previous%20sum%3F%20%28Hint%3A%20it%20has%0A%2F%2F%20to%20do%20with%20the%20borrowing%20rules.%29%0A%0A%2F%2F%20Exercise%203.%20How%20might%20you%20modify%20the%20trait%20to%20avoid%20this%0A%2F%2F%20intermediate%3F%0A%2F%2F%0A%2F%2F%20Hint%20i.%20%60%26mut%60%0A%2F%2F%20Hint%20ii.%20%60Copy%60%0A
[play70]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Structs%2C%20enums%2C%20and%20inherent%20methods.%0A%0Aconst%20PI%3A%20f64%20%3D%203.14159%3B%0A%0A%23%5Bderiving%28Show%29%5D%20%20%20%20%20%20%20%20%20%20%20%20%0Astruct%20Point%20%7B%0A%20%20%20%20x%3A%20f64%2C%0A%20%20%20%20y%3A%20f64%2C%0A%7D%0A%0A%23%5Bderiving%28Show%29%5D%0Aenum%20Shape%20%7B%20%20%20%20%20%0A%0A%20%20%20%20Circle%28Point%2C%20f64%29%2C%20%20%20%20%20%0A%0A%20%20%20%20Rectangle%28%20Point%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Point%29%0A%7D%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20origin%20%3D%20Point%20%7B%20x%3A%200.0%2C%20y%3A%200.0%2C%20%7D%3B%20%20%20%0A%0A%20%20%20%20let%20unit%20%3D%20Point%20%7B%20x%3A%201.0%2C%20y%3A%201.0%2C%20%7D%3B%0A%0A%20%20%20%20let%20mut%20shape%20%3D%20Circle%28origin%2C%2022.0%29%3B%0A%20%20%20%20println%21%28%22Area%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%20shape%2C%20shape.area%28%29%29%3B%0A%0A%20%20%20%20shape.enlarge%283.5%29%3B%0A%20%20%20%20println%21%28%22Area%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%20%28enlarged%29%22%2C%20shape%2C%20shape.area%28%29%29%3B%0A%0A%20%20%20%20shape%20%3D%20Rectangle%28origin%2C%20unit%29%3B%0A%20%20%20%20println%21%28%22Area%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%20shape%2C%20shape.area%28%29%29%3B%0A%7D%0A%0Aimpl%20Shape%20%7B%0A%20%20%20%20fn%20area%28%26self%29%20-%3E%20f64%20%7B%0A%20%20%20%20%20%20%20%20match%20%2Aself%20%7B%20%20%20%0A%0A%20%20%20%20%20%20%20%20%20%20%20%20Circle%28_%2C%20radius%29%20%3D%3E%202.0%20%2A%20PI%20%2A%20radius%2C%20%20%20%0A%0A%20%20%20%20%20%20%20%20%20%20%20%20Rectangle%28ref%20ul%2C%20ref%20lr%29%20%3D%3E%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%28lr.x%20-%20ul.x%29.abs%28%29%20%2A%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%28lr.y%20-%20ul.y%29.abs%28%29%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%0A%20%20%20%20fn%20enlarge%28%26mut%20self%2C%20scale%3A%20f64%29%20%7B%0A%20%20%20%20%20%20%20%20match%20%2Aself%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20Circle%28_%2C%20ref%20mut%20radius%29%20%3D%3E%20%2Aradius%20%2A%3D%20scale%2C%20%20%0A%0A%20%20%20%20%20%20%20%20%20%20%20%20Rectangle%28ref%20_ul%2C%20ref%20mut%20_lr%29%20%3D%3E%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20fail%21%28%22Math%20is%20hard%22%29%0A%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Remove%20one%20of%20the%20variants%20above.%20What%20happens%3F%0A%0A%2F%2F%20Exercise%202.%20Complete%20the%20%60Rectangle%60%20case%20of%20%60enlarge%60.%0A
[play80]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Memory%20allocation%20and%20ownership.%0A%0Ause%20std%3A%3Afmt%3B%0A%0Astruct%20List%3CT%3E%20%7B%0A%20%20%20%20data%3A%20T%2C%0A%20%20%20%20next%3A%20Option%3CBox%3CList%3CT%3E%3E%3E%2C%20%20%20%20%20%20%0A%7D%0A%0A%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20x%20%3D%20List%3A%3Anew%2844i%29%3B%0A%20%20%20%20x%20%3D%20x.prepend%2822%29%3B%0A%20%20%20%20x%20%3D%20x.append%2866%29%3B%0A%20%20%20%20println%21%28%22x%3D%7B%7D%22%2C%20x%29%3B%0A%7D%0A%0Aimpl%3CT%3E%20List%3CT%3E%20%7B%0A%20%20%20%20fn%20new%28value%3A%20T%29%20-%3E%20List%3CT%3E%20%7B%0A%20%20%20%20%20%20%20%20List%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20data%3A%20value%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20next%3A%20None%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%0A%20%20%20%20fn%20prepend%28self%2C%20value%3A%20T%29%20-%3E%20List%3CT%3E%20%7B%0A%20%20%20%20%20%20%20%20List%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20data%3A%20value%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20next%3A%20Some%28box%20self%29%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%0A%20%20%20%20fn%20append%28self%2C%20value%3A%20T%29%20-%3E%20List%3CT%3E%20%7B%0A%20%20%20%20%20%20%20%20let%20next%20%3D%20match%20self.next%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20None%20%3D%3E%20List%3A%3Anew%28value%29%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20Some%28list%29%20%3D%3E%20list.append%28value%29%2C%0A%20%20%20%20%20%20%20%20%7D%3B%0A%0A%20%20%20%20%20%20%20%20List%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20data%3A%20self.data%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20next%3A%20Some%28box%20next%29%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%7D%0A%0Aimpl%3CT%3Afmt%3A%3AShow%3E%20fmt%3A%3AShow%20for%20List%3CT%3E%20%7B%0A%20%20%20%20fn%20fmt%28%26self%2C%20fmt%3A%20%26mut%20fmt%3A%3AFormatter%29%20-%3E%20fmt%3A%3AResult%20%7B%0A%20%20%20%20%20%20%20%20try%21%28write%21%28fmt%2C%20%22%5B%7B%7D%22%2C%20self.data%29%29%3B%0A%0A%20%20%20%20%20%20%20%20let%20mut%20pointer%20%3D%20%26self.next%3B%0A%20%20%20%20%20%20%20%20loop%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20match%20%2Apointer%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20None%20%3D%3E%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20return%20write%21%28fmt%2C%20%22%5D%22%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20Some%28box%20ref%20p%29%20%3D%3E%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20try%21%28write%21%28fmt%2C%20%22%2C%20%7B%7D%22%2C%20p.data%29%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20pointer%20%3D%20%26p.next%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Rewrite%20%60append%60%20to%20avoid%20reallocating%20the%20entire%0A%2F%2F%20vector.%0A%0A%2F%2F%20Exercise%202.%20The%20methods%20%60prepend%60%20and%20%60append%60%20take%20ownership%20of%0A%2F%2F%20the%20list.%20What%20is%20the%20downside%20of%20this%3F%20Can%20you%20rewrite%20%60prepend%60%0A%2F%2F%20and%20%60append%60%20to%20take%20%60%26mut%20self%60%20instead%3F%0A%2F%2F%0A%2F%2F%20Hint%20i.%20Consider%20%60std%3A%3Amem%3A%3Aswap%60.%0A
[play90]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Using%20iterators.%0A%0Ause%20std%3A%3Aiter%3A%3AAdditiveIterator%3B%20%20%20%20%20%20%20%20%20%20%20%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20vec1%3A%20Vec%3Cint%3E%20%3D%20vec%21%5B22%2C%2044%2C%2066%5D%3B%0A%20%20%20%20let%20vec2%3A%20Vec%3Cint%3E%20%3D%20vec%21%5B44%2C%2066%2C%2088%5D%3B%0A%0A%20%20%20%20println%21%28%22sum%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20vec1%2C%20sum%28vec1%5B%5D%29%29%3B%0A%0A%20%20%20%20println%21%28%22sum%20of%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20vec2%2C%20sum%28vec2%5B%5D%29%29%3B%0A%0A%20%20%20%20println%21%28%22%60%7B%7D%60%20dot%20%60%7B%7D%60%20is%20%60%7B%7D%60%22%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20vec1%2C%20vec2%2C%20dot_product%28vec1%5B%5D%2C%20vec2%5B%5D%29%29%3B%0A%7D%0A%0Afn%20sum%28ints%3A%20%26%5Bint%5D%29%20-%3E%20int%20%7B%0A%20%20%20%20let%20mut%20sum%20%3D%200%3B%0A%20%20%20%20for%20%26i%20in%20ints.iter%28%29%20%7B%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20%20%20%20%20sum%20%2B%3D%20i%3B%0A%20%20%20%20%7D%0A%20%20%20%20sum%0A%7D%0A%0Afn%20dot_product%28vec1%3A%20%26%5Bint%5D%2C%20vec2%3A%20%26%5Bint%5D%29%20-%3E%20int%20%7B%0A%20%20%20%20vec1.iter%28%29%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20-%3E%20Iterator%3C%26int%3E%0A%20%20%20%20%20%20%20%20.zip%28vec2.iter%28%29%29%20%20%20%20%20%20%20%2F%2F%20-%3E%20Iterator%3C%28%26int%2C%26int%29%3E%0A%20%20%20%20%20%20%20%20.map%28%7C%28%26a%2C%20%26b%29%7C%20a%20%2A%20b%29%20%20%2F%2F%20-%3E%20Iterator%3Cint%3E%0A%20%20%20%20%20%20%20%20%0A%20%20%20%20%20%20%20%20.sum%28%29%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%7D%0A
[play100]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Defining%20iterators%3B%20named%20borrow%20scopes.%0A%0A%23%5Bderiving%28Show%29%5D%0Astruct%20List%3CT%3E%20%7B%0A%20%20%20%20data%3A%20T%2C%0A%20%20%20%20next%3A%20Option%3CBox%3CList%3CT%3E%3E%3E%2C%0A%7D%0A%0A%23%5Bderiving%28Show%29%5D%0Astruct%20ListIterator%3C%27a%2C%20T%3A%27a%3E%20%7B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%0A%20%20%20%20list%3A%20%26%27a%20List%3CT%3E%2C%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%0A%7D%0A%0Apub%20fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20x%20%3D%20List%3A%3Anew%2866i%29%3B%0A%20%20%20%20x%20%3D%20x.prepend%2844%29%3B%0A%20%20%20%20x%20%3D%20x.prepend%2822%29%3B%0A%0A%20%20%20%20println%21%28%22List%20is%20%7B%7D%22%2C%20x%29%3B%0A%20%20%20%20for%20%28idx%2C%20elem%29%20in%20x.iter%28%29.enumerate%28%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22Element%20%23%7B%7D%20is%20%60%7B%7D%60%22%2C%20idx%2C%20elem%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Aimpl%3CT%3E%20List%3CT%3E%20%7B%0A%20%20%20%20fn%20new%28value%3A%20T%29%20-%3E%20List%3CT%3E%20%7B%0A%20%20%20%20%20%20%20%20List%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20data%3A%20value%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20next%3A%20None%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%0A%20%20%20%20fn%20prepend%28self%2C%20value%3A%20T%29%20-%3E%20List%3CT%3E%20%7B%0A%20%20%20%20%20%20%20%20List%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20data%3A%20value%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20next%3A%20Some%28box%20self%29%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%0A%20%20%20%20fn%20iter%28%26self%29%20-%3E%20ListIterator%3CT%3E%20%7B%20%20%20%0A%0A%20%20%20%20%20%20%20%20ListIterator%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20list%3A%20self%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%7D%0A%0Aimpl%3C%27a%2CT%3E%20Iterator%3C%26%27a%20T%3E%20for%20ListIterator%3C%27a%2CT%3E%20%7B%20%0A%0A%20%20%20%20fn%20next%28%26mut%20self%29%20-%3E%20Option%3C%26%27a%20T%3E%20%7B%0A%20%20%20%20%20%20%20%20None%20%2F%2F%20Um...%0A%20%20%20%20%7D%0A%7D%0A%0A%2F%2F%20Exercise%201.%20Implement%20the%20iterator.%0A%0A%2F%2F%20Exercise%202%20%28extra%20credit%29.%20Implement%20an%20iterator%20over%20mutable%20references.%0A
[play110]: http://play.rust-lang.org/?code=%2F%2F%20Theme%3A%20Shared%20ownership%20and%20mutation.%0A%0Ause%20std%3A%3Arc%3A%3ARc%3B%0A%0Astruct%20Configuration%20%7B%0A%20%20%20%20installed_packages%3A%20Vec%3CRc%3CPackage%3E%3E%0A%7D%0A%0Astruct%20Room%20%7B%0A%20%20%20%20players%3A%20Vec%3CRc%3CPlayer%3E%3E%2C%0A%7D%0A%0Apub%20fn%20main%28%29%20%7B%20%7D%0A