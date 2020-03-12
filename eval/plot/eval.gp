# solution1.gnuplot
reset
# set terminal pngcairo  transparent enhanced font "arial,10" fontscale 1.0 size 600, 400 
# set output 'eval.png'
set terminal pdf size 7, 2.5
set output 'eval.pdf'

set title "N3X rewrites"
set key outside below

set xrange [0.5:16.5]
set yrange [0:1]
set ylabel "Triples relative to N3 triples"
set xtics 1

set style fill solid border -1

num_of_categories=2
set boxwidth 0.6/num_of_categories
offset=0.15

plot 'eval.n3.dat'  using ($1-offset):($3+$4) title "N3 triples" linecolor rgb 'dark-orange' with boxes fs pattern 4, \
     ''             using ($1-offset):4 title "N3 math" linecolor rgb 'dark-orange' with boxes fs pattern 6, \
     'eval.n3x.dat' using ($1+offset):($3+$4) title "N3X triples" linecolor rgb 'steelblue' with boxes fs pattern 5, \
     ''             using ($1+offset):4 title "N3X math" linecolor rgb 'steelblue' with boxes fs pattern 7