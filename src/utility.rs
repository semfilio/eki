pub fn friction_factor( relative: f64, reynolds: f64 ) -> f64 {
    if reynolds < 2100.0 {
        64.0 / reynolds 
    } else if reynolds > 3000.0 {
        //TODO Praks-Brkic ( temporary )
        let a = reynolds * relative / 8.0897;
        let b = reynolds.ln() - 0.779626;
        let x = a + b;
        let c = x.ln();
        let k = 0.8685972 * ( b - c + ( c / ( x - 0.5588 * c + 1.2079 ) ) );
        1.0 / ( k * k )
    } else {
        let k1 = (64.0/reynolds).powi( 12 );
        let c = 1.0 / ( (0.833* reynolds.powf(1.282) / reynolds.powf(1.007) ) + ( 0.27 * relative ) 
        + (110.0*relative/ reynolds ));
        let a = 0.8687 * (c.powi( 16 )).ln();
        let b = ( 13269.0 / reynolds ).powi( 16 );
        let k2 = (a+b).powf( -1.5 );
        (k1+k2).powf( 0.08333333333 )
    }
}