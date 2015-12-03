mgg13TeV[1000,5000];

jj_RS1ZZ_sig_m0[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha[0.8, 0.5, 3]; 
jj_RS1ZZ_sig_n[13.0, 0.5, 10]; 
jj_RS1ZZ_sig_gsigma[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_gsigma);
jjCBSigRS1ZZ    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_sigma, jj_RS1ZZ_sig_alpha, jj_RS1ZZ_sig_n);
RS1ZZ_jj      = AddPdf(jjGaussSigRS1ZZ, jjCBSigRS1ZZ, jj_RS1ZZ_sig_frac);

jj_RS1ZZ_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];

jj_RS1ZZ_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];

jj_RS1ZZ_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];

jj_RS1ZZ_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];


jjGaussSigRS1ZZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1ZZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_sigma_CMS_jj_VVHP, jj_RS1ZZ_sig_alpha_CMS_jj_VVHP, jj_RS1ZZ_sig_n_CMS_jj_VVHP);
RS1ZZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVHP, jjCBSigRS1ZZ_CMS_jj_VVHP, jj_RS1ZZ_sig_frac_CMS_jj_VVHP);

jjGaussSigRS1ZZ_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWHP, jj_RS1ZZ_sig_gsigma_CMS_jj_WWHP);
jjCBSigRS1ZZ_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWHP, jj_RS1ZZ_sig_sigma_CMS_jj_WWHP, jj_RS1ZZ_sig_alpha_CMS_jj_WWHP, jj_RS1ZZ_sig_n_CMS_jj_WWHP);
RS1ZZ_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WWHP, jjCBSigRS1ZZ_CMS_jj_WWHP, jj_RS1ZZ_sig_frac_CMS_jj_WWHP);

jjGaussSigRS1ZZ_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZHP, jj_RS1ZZ_sig_gsigma_CMS_jj_WZHP);
jjCBSigRS1ZZ_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZHP, jj_RS1ZZ_sig_sigma_CMS_jj_WZHP, jj_RS1ZZ_sig_alpha_CMS_jj_WZHP, jj_RS1ZZ_sig_n_CMS_jj_WZHP);
RS1ZZ_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WZHP, jjCBSigRS1ZZ_CMS_jj_WZHP, jj_RS1ZZ_sig_frac_CMS_jj_WZHP);

jjGaussSigRS1ZZ_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZHP, jj_RS1ZZ_sig_gsigma_CMS_jj_ZZHP);
jjCBSigRS1ZZ_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZHP, jj_RS1ZZ_sig_sigma_CMS_jj_ZZHP, jj_RS1ZZ_sig_alpha_CMS_jj_ZZHP, jj_RS1ZZ_sig_n_CMS_jj_ZZHP);
RS1ZZ_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_ZZHP, jjCBSigRS1ZZ_CMS_jj_ZZHP, jj_RS1ZZ_sig_frac_CMS_jj_ZZHP);


jj_RS1ZZ_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1ZZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_sigma_CMS_jj_VVLP, jj_RS1ZZ_sig_alpha_CMS_jj_VVLP, jj_RS1ZZ_sig_n_CMS_jj_VVLP);
RS1ZZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVLP, jjCBSigRS1ZZ_CMS_jj_VVLP, jj_RS1ZZ_sig_frac_CMS_jj_VVLP);


jj_RS1ZZ_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWLP, jj_RS1ZZ_sig_gsigma_CMS_jj_WWLP);
jjCBSigRS1ZZ_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWLP, jj_RS1ZZ_sig_sigma_CMS_jj_WWLP, jj_RS1ZZ_sig_alpha_CMS_jj_WWLP, jj_RS1ZZ_sig_n_CMS_jj_WWLP);
RS1ZZ_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WWLP, jjCBSigRS1ZZ_CMS_jj_WWLP, jj_RS1ZZ_sig_frac_CMS_jj_WWLP);


jj_RS1ZZ_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZLP, jj_RS1ZZ_sig_gsigma_CMS_jj_WZLP);
jjCBSigRS1ZZ_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZLP, jj_RS1ZZ_sig_sigma_CMS_jj_WZLP, jj_RS1ZZ_sig_alpha_CMS_jj_WZLP, jj_RS1ZZ_sig_n_CMS_jj_WZLP);
RS1ZZ_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WZLP, jjCBSigRS1ZZ_CMS_jj_WZLP, jj_RS1ZZ_sig_frac_CMS_jj_WZLP);

jj_RS1ZZ_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZLP, jj_RS1ZZ_sig_gsigma_CMS_jj_ZZLP);
jjCBSigRS1ZZ_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZLP, jj_RS1ZZ_sig_sigma_CMS_jj_ZZLP, jj_RS1ZZ_sig_alpha_CMS_jj_ZZLP, jj_RS1ZZ_sig_n_CMS_jj_ZZLP);
RS1ZZ_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_ZZLP, jjCBSigRS1ZZ_CMS_jj_ZZLP, jj_RS1ZZ_sig_frac_CMS_jj_ZZLP);


jj_RS1ZZ_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1ZZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_sigma_CMS_jj_VVNP, jj_RS1ZZ_sig_alpha_CMS_jj_VVNP, jj_RS1ZZ_sig_n_CMS_jj_VVNP);
RS1ZZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVNP, jjCBSigRS1ZZ_CMS_jj_VVNP, jj_RS1ZZ_sig_frac_CMS_jj_VVNP);


jj_RS1ZZ_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWNP, jj_RS1ZZ_sig_gsigma_CMS_jj_WWNP);
jjCBSigRS1ZZ_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WWNP, jj_RS1ZZ_sig_sigma_CMS_jj_WWNP, jj_RS1ZZ_sig_alpha_CMS_jj_WWNP, jj_RS1ZZ_sig_n_CMS_jj_WWNP);
RS1ZZ_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WWNP, jjCBSigRS1ZZ_CMS_jj_WWNP, jj_RS1ZZ_sig_frac_CMS_jj_WWNP);


jj_RS1ZZ_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZNP, jj_RS1ZZ_sig_gsigma_CMS_jj_WZNP);
jjCBSigRS1ZZ_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_WZNP, jj_RS1ZZ_sig_sigma_CMS_jj_WZNP, jj_RS1ZZ_sig_alpha_CMS_jj_WZNP, jj_RS1ZZ_sig_n_CMS_jj_WZNP);
RS1ZZ_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_WZNP, jjCBSigRS1ZZ_CMS_jj_WZNP, jj_RS1ZZ_sig_frac_CMS_jj_WZNP);


jj_RS1ZZ_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZNP, jj_RS1ZZ_sig_gsigma_CMS_jj_ZZNP);
jjCBSigRS1ZZ_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_ZZNP, jj_RS1ZZ_sig_sigma_CMS_jj_ZZNP, jj_RS1ZZ_sig_alpha_CMS_jj_ZZNP, jj_RS1ZZ_sig_n_CMS_jj_ZZNP);
RS1ZZ_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_ZZNP, jjCBSigRS1ZZ_CMS_jj_ZZNP, jj_RS1ZZ_sig_frac_CMS_jj_ZZNP);



jj_RS1ZZ_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1ZZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_sigma_CMS_jj_qVHP, jj_RS1ZZ_sig_alpha_CMS_jj_qVHP, jj_RS1ZZ_sig_n_CMS_jj_qVHP);
RS1ZZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVHP, jjCBSigRS1ZZ_CMS_jj_qVHP, jj_RS1ZZ_sig_frac_CMS_jj_qVHP);

jj_RS1ZZ_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWHP, jj_RS1ZZ_sig_gsigma_CMS_jj_qWHP);
jjCBSigRS1ZZ_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWHP, jj_RS1ZZ_sig_sigma_CMS_jj_qWHP, jj_RS1ZZ_sig_alpha_CMS_jj_qWHP, jj_RS1ZZ_sig_n_CMS_jj_qWHP);
RS1ZZ_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qWHP, jjCBSigRS1ZZ_CMS_jj_qWHP, jj_RS1ZZ_sig_frac_CMS_jj_qWHP);

jj_RS1ZZ_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZHP, jj_RS1ZZ_sig_gsigma_CMS_jj_qZHP);
jjCBSigRS1ZZ_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZHP, jj_RS1ZZ_sig_sigma_CMS_jj_qZHP, jj_RS1ZZ_sig_alpha_CMS_jj_qZHP, jj_RS1ZZ_sig_n_CMS_jj_qZHP);
RS1ZZ_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qZHP, jjCBSigRS1ZZ_CMS_jj_qZHP, jj_RS1ZZ_sig_frac_CMS_jj_qZHP);


jj_RS1ZZ_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1ZZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_sigma_CMS_jj_qVLP, jj_RS1ZZ_sig_alpha_CMS_jj_qVLP, jj_RS1ZZ_sig_n_CMS_jj_qVLP);
RS1ZZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVLP, jjCBSigRS1ZZ_CMS_jj_qVLP, jj_RS1ZZ_sig_frac_CMS_jj_qVLP);

jj_RS1ZZ_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWLP, jj_RS1ZZ_sig_gsigma_CMS_jj_qWLP);
jjCBSigRS1ZZ_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWLP, jj_RS1ZZ_sig_sigma_CMS_jj_qWLP, jj_RS1ZZ_sig_alpha_CMS_jj_qWLP, jj_RS1ZZ_sig_n_CMS_jj_qWLP);
RS1ZZ_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qWLP, jjCBSigRS1ZZ_CMS_jj_qWLP, jj_RS1ZZ_sig_frac_CMS_jj_qWLP);

jj_RS1ZZ_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZLP, jj_RS1ZZ_sig_gsigma_CMS_jj_qZLP);
jjCBSigRS1ZZ_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZLP, jj_RS1ZZ_sig_sigma_CMS_jj_qZLP, jj_RS1ZZ_sig_alpha_CMS_jj_qZLP, jj_RS1ZZ_sig_n_CMS_jj_qZLP);
RS1ZZ_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qZLP, jjCBSigRS1ZZ_CMS_jj_qZLP, jj_RS1ZZ_sig_frac_CMS_jj_qZLP);


jj_RS1ZZ_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1ZZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_sigma_CMS_jj_qVNP, jj_RS1ZZ_sig_alpha_CMS_jj_qVNP, jj_RS1ZZ_sig_n_CMS_jj_qVNP);
RS1ZZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVNP, jjCBSigRS1ZZ_CMS_jj_qVNP, jj_RS1ZZ_sig_frac_CMS_jj_qVNP);

jj_RS1ZZ_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWNP, jj_RS1ZZ_sig_gsigma_CMS_jj_qWNP);
jjCBSigRS1ZZ_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qWNP, jj_RS1ZZ_sig_sigma_CMS_jj_qWNP, jj_RS1ZZ_sig_alpha_CMS_jj_qWNP, jj_RS1ZZ_sig_n_CMS_jj_qWNP);
RS1ZZ_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qWNP, jjCBSigRS1ZZ_CMS_jj_qWNP, jj_RS1ZZ_sig_frac_CMS_jj_qWNP);


jj_RS1ZZ_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4000.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZNP, jj_RS1ZZ_sig_gsigma_CMS_jj_qZNP);
jjCBSigRS1ZZ_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qZNP, jj_RS1ZZ_sig_sigma_CMS_jj_qZNP, jj_RS1ZZ_sig_alpha_CMS_jj_qZNP, jj_RS1ZZ_sig_n_CMS_jj_qZNP);
RS1ZZ_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qZNP, jjCBSigRS1ZZ_CMS_jj_qZNP, jj_RS1ZZ_sig_frac_CMS_jj_qZNP);






jj_RS1WW_sig_m0[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha[0.8, 0.5, 3]; 
jj_RS1WW_sig_n[13.0, 0.5, 10]; 
jj_RS1WW_sig_gsigma[100, 0.0, 1000.0];
jj_RS1WW_sig_frac[0.5, 0.0, 1.0];

jjGaussSigRS1WW = Gaussian(mgg13TeV, jj_RS1WW_sig_m0, jj_RS1WW_sig_gsigma);
jjCBSigRS1WW    = CBShape(mgg13TeV, jj_RS1WW_sig_m0, jj_RS1WW_sig_sigma, jj_RS1WW_sig_alpha, jj_RS1WW_sig_n);
RS1WW_jj      = AddPdf(jjGaussSigRS1WW, jjCBSigRS1WW, jj_RS1WW_sig_frac);

jj_RS1WW_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1WW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_sigma_CMS_jj_VVHP, jj_RS1WW_sig_alpha_CMS_jj_VVHP, jj_RS1WW_sig_n_CMS_jj_VVHP);
RS1WW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVHP, jjCBSigRS1WW_CMS_jj_VVHP, jj_RS1WW_sig_frac_CMS_jj_VVHP);

jj_RS1WW_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWHP, jj_RS1WW_sig_gsigma_CMS_jj_WWHP);
jjCBSigRS1WW_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWHP, jj_RS1WW_sig_sigma_CMS_jj_WWHP, jj_RS1WW_sig_alpha_CMS_jj_WWHP, jj_RS1WW_sig_n_CMS_jj_WWHP);
RS1WW_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WWHP, jjCBSigRS1WW_CMS_jj_WWHP, jj_RS1WW_sig_frac_CMS_jj_WWHP);

jj_RS1WW_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZHP, jj_RS1WW_sig_gsigma_CMS_jj_WZHP);
jjCBSigRS1WW_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZHP, jj_RS1WW_sig_sigma_CMS_jj_WZHP, jj_RS1WW_sig_alpha_CMS_jj_WZHP, jj_RS1WW_sig_n_CMS_jj_WZHP);
RS1WW_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WZHP, jjCBSigRS1WW_CMS_jj_WZHP, jj_RS1WW_sig_frac_CMS_jj_WZHP);

jj_RS1WW_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZHP, jj_RS1WW_sig_gsigma_CMS_jj_ZZHP);
jjCBSigRS1WW_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZHP, jj_RS1WW_sig_sigma_CMS_jj_ZZHP, jj_RS1WW_sig_alpha_CMS_jj_ZZHP, jj_RS1WW_sig_n_CMS_jj_ZZHP);
RS1WW_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_ZZHP, jjCBSigRS1WW_CMS_jj_ZZHP, jj_RS1WW_sig_frac_CMS_jj_ZZHP);






jj_RS1WW_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1WW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_sigma_CMS_jj_VVLP, jj_RS1WW_sig_alpha_CMS_jj_VVLP, jj_RS1WW_sig_n_CMS_jj_VVLP);
RS1WW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVLP, jjCBSigRS1WW_CMS_jj_VVLP, jj_RS1WW_sig_frac_CMS_jj_VVLP);

jj_RS1WW_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWLP, jj_RS1WW_sig_gsigma_CMS_jj_WWLP);
jjCBSigRS1WW_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWLP, jj_RS1WW_sig_sigma_CMS_jj_WWLP, jj_RS1WW_sig_alpha_CMS_jj_WWLP, jj_RS1WW_sig_n_CMS_jj_WWLP);
RS1WW_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WWLP, jjCBSigRS1WW_CMS_jj_WWLP, jj_RS1WW_sig_frac_CMS_jj_WWLP);

jj_RS1WW_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZLP, jj_RS1WW_sig_gsigma_CMS_jj_WZLP);
jjCBSigRS1WW_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZLP, jj_RS1WW_sig_sigma_CMS_jj_WZLP, jj_RS1WW_sig_alpha_CMS_jj_WZLP, jj_RS1WW_sig_n_CMS_jj_WZLP);
RS1WW_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WZLP, jjCBSigRS1WW_CMS_jj_WZLP, jj_RS1WW_sig_frac_CMS_jj_WZLP);

jj_RS1WW_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZLP, jj_RS1WW_sig_gsigma_CMS_jj_ZZLP);
jjCBSigRS1WW_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZLP, jj_RS1WW_sig_sigma_CMS_jj_ZZLP, jj_RS1WW_sig_alpha_CMS_jj_ZZLP, jj_RS1WW_sig_n_CMS_jj_ZZLP);
RS1WW_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_ZZLP, jjCBSigRS1WW_CMS_jj_ZZLP, jj_RS1WW_sig_frac_CMS_jj_ZZLP);




jj_RS1WW_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1WW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_sigma_CMS_jj_VVNP, jj_RS1WW_sig_alpha_CMS_jj_VVNP, jj_RS1WW_sig_n_CMS_jj_VVNP);
RS1WW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVNP, jjCBSigRS1WW_CMS_jj_VVNP, jj_RS1WW_sig_frac_CMS_jj_VVNP);

jj_RS1WW_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWNP, jj_RS1WW_sig_gsigma_CMS_jj_WWNP);
jjCBSigRS1WW_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WWNP, jj_RS1WW_sig_sigma_CMS_jj_WWNP, jj_RS1WW_sig_alpha_CMS_jj_WWNP, jj_RS1WW_sig_n_CMS_jj_WWNP);
RS1WW_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WWNP, jjCBSigRS1WW_CMS_jj_WWNP, jj_RS1WW_sig_frac_CMS_jj_WWNP);

jj_RS1WW_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZNP, jj_RS1WW_sig_gsigma_CMS_jj_WZNP);
jjCBSigRS1WW_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_WZNP, jj_RS1WW_sig_sigma_CMS_jj_WZNP, jj_RS1WW_sig_alpha_CMS_jj_WZNP, jj_RS1WW_sig_n_CMS_jj_WZNP);
RS1WW_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_WZNP, jjCBSigRS1WW_CMS_jj_WZNP, jj_RS1WW_sig_frac_CMS_jj_WZNP);

jj_RS1WW_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZNP, jj_RS1WW_sig_gsigma_CMS_jj_ZZNP);
jjCBSigRS1WW_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_ZZNP, jj_RS1WW_sig_sigma_CMS_jj_ZZNP, jj_RS1WW_sig_alpha_CMS_jj_ZZNP, jj_RS1WW_sig_n_CMS_jj_ZZNP);
RS1WW_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_ZZNP, jjCBSigRS1WW_CMS_jj_ZZNP, jj_RS1WW_sig_frac_CMS_jj_ZZNP);



jj_RS1WW_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1WW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_sigma_CMS_jj_qVHP, jj_RS1WW_sig_alpha_CMS_jj_qVHP, jj_RS1WW_sig_n_CMS_jj_qVHP);
RS1WW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVHP, jjCBSigRS1WW_CMS_jj_qVHP, jj_RS1WW_sig_frac_CMS_jj_qVHP);

jj_RS1WW_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWHP, jj_RS1WW_sig_gsigma_CMS_jj_qWHP);
jjCBSigRS1WW_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWHP, jj_RS1WW_sig_sigma_CMS_jj_qWHP, jj_RS1WW_sig_alpha_CMS_jj_qWHP, jj_RS1WW_sig_n_CMS_jj_qWHP);
RS1WW_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qWHP, jjCBSigRS1WW_CMS_jj_qWHP, jj_RS1WW_sig_frac_CMS_jj_qWHP);

jj_RS1WW_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZHP, jj_RS1WW_sig_gsigma_CMS_jj_qZHP);
jjCBSigRS1WW_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZHP, jj_RS1WW_sig_sigma_CMS_jj_qZHP, jj_RS1WW_sig_alpha_CMS_jj_qZHP, jj_RS1WW_sig_n_CMS_jj_qZHP);
RS1WW_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qZHP, jjCBSigRS1WW_CMS_jj_qZHP, jj_RS1WW_sig_frac_CMS_jj_qZHP);



jj_RS1WW_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1WW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_sigma_CMS_jj_qVLP, jj_RS1WW_sig_alpha_CMS_jj_qVLP, jj_RS1WW_sig_n_CMS_jj_qVLP);
RS1WW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVLP, jjCBSigRS1WW_CMS_jj_qVLP, jj_RS1WW_sig_frac_CMS_jj_qVLP);

jj_RS1WW_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWLP, jj_RS1WW_sig_gsigma_CMS_jj_qWLP);
jjCBSigRS1WW_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWLP, jj_RS1WW_sig_sigma_CMS_jj_qWLP, jj_RS1WW_sig_alpha_CMS_jj_qWLP, jj_RS1WW_sig_n_CMS_jj_qWLP);
RS1WW_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qWLP, jjCBSigRS1WW_CMS_jj_qWLP, jj_RS1WW_sig_frac_CMS_jj_qWLP);

jj_RS1WW_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZLP, jj_RS1WW_sig_gsigma_CMS_jj_qZLP);
jjCBSigRS1WW_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZLP, jj_RS1WW_sig_sigma_CMS_jj_qZLP, jj_RS1WW_sig_alpha_CMS_jj_qZLP, jj_RS1WW_sig_n_CMS_jj_qZLP);
RS1WW_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qZLP, jjCBSigRS1WW_CMS_jj_qZLP, jj_RS1WW_sig_frac_CMS_jj_qZLP);




jj_RS1WW_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1WW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_sigma_CMS_jj_qVNP, jj_RS1WW_sig_alpha_CMS_jj_qVNP, jj_RS1WW_sig_n_CMS_jj_qVNP);
RS1WW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVNP, jjCBSigRS1WW_CMS_jj_qVNP, jj_RS1WW_sig_frac_CMS_jj_qVNP);

jj_RS1WW_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWNP, jj_RS1WW_sig_gsigma_CMS_jj_qWNP);
jjCBSigRS1WW_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qWNP, jj_RS1WW_sig_sigma_CMS_jj_qWNP, jj_RS1WW_sig_alpha_CMS_jj_qWNP, jj_RS1WW_sig_n_CMS_jj_qWNP);
RS1WW_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qWNP, jjCBSigRS1WW_CMS_jj_qWNP, jj_RS1WW_sig_frac_CMS_jj_qWNP);

jj_RS1WW_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4000.0];
jj_RS1WW_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigRS1WW_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZNP, jj_RS1WW_sig_gsigma_CMS_jj_qZNP);
jjCBSigRS1WW_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qZNP, jj_RS1WW_sig_sigma_CMS_jj_qZNP, jj_RS1WW_sig_alpha_CMS_jj_qZNP, jj_RS1WW_sig_n_CMS_jj_qZNP);
RS1WW_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qZNP, jjCBSigRS1WW_CMS_jj_qZNP, jj_RS1WW_sig_frac_CMS_jj_qZNP);


jj_WZ_sig_m0[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma[100, 0.0, 1000.0];
jj_WZ_sig_alpha[0.8, 0.5, 3]; 
jj_WZ_sig_n[13.0, 0.5, 10]; 
jj_WZ_sig_gsigma[100, 0.0, 1000.0];
jj_WZ_sig_frac[0.5, 0.0, 1.0];

jjGaussSigWZ = Gaussian(mgg13TeV, jj_WZ_sig_m0, jj_WZ_sig_gsigma);
jjCBSigWZ    = CBShape(mgg13TeV, jj_WZ_sig_m0, jj_WZ_sig_sigma, jj_WZ_sig_alpha, jj_WZ_sig_n);
WZ_jj      = AddPdf(jjGaussSigWZ, jjCBSigWZ, jj_WZ_sig_frac);

jj_WZ_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];


jjGaussSigWZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigWZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_sigma_CMS_jj_VVHP, jj_WZ_sig_alpha_CMS_jj_VVHP, jj_WZ_sig_n_CMS_jj_VVHP);
WZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigWZ_CMS_jj_VVHP, jjCBSigWZ_CMS_jj_VVHP, jj_WZ_sig_frac_CMS_jj_VVHP);


jj_WZ_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];


jjGaussSigWZ_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWHP, jj_WZ_sig_gsigma_CMS_jj_WWHP);
jjCBSigWZ_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWHP, jj_WZ_sig_sigma_CMS_jj_WWHP, jj_WZ_sig_alpha_CMS_jj_WWHP, jj_WZ_sig_n_CMS_jj_WWHP);
WZ_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigWZ_CMS_jj_WWHP, jjCBSigWZ_CMS_jj_WWHP, jj_WZ_sig_frac_CMS_jj_WWHP);


jj_WZ_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];


jjGaussSigWZ_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZHP, jj_WZ_sig_gsigma_CMS_jj_WZHP);
jjCBSigWZ_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZHP, jj_WZ_sig_sigma_CMS_jj_WZHP, jj_WZ_sig_alpha_CMS_jj_WZHP, jj_WZ_sig_n_CMS_jj_WZHP);
WZ_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigWZ_CMS_jj_WZHP, jjCBSigWZ_CMS_jj_WZHP, jj_WZ_sig_frac_CMS_jj_WZHP);

jj_WZ_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];


jjGaussSigWZ_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZHP, jj_WZ_sig_gsigma_CMS_jj_ZZHP);
jjCBSigWZ_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZHP, jj_WZ_sig_sigma_CMS_jj_ZZHP, jj_WZ_sig_alpha_CMS_jj_ZZHP, jj_WZ_sig_n_CMS_jj_ZZHP);
WZ_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigWZ_CMS_jj_ZZHP, jjCBSigWZ_CMS_jj_ZZHP, jj_WZ_sig_frac_CMS_jj_ZZHP);



jj_WZ_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigWZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_sigma_CMS_jj_VVLP, jj_WZ_sig_alpha_CMS_jj_VVLP, jj_WZ_sig_n_CMS_jj_VVLP);
WZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigWZ_CMS_jj_VVLP, jjCBSigWZ_CMS_jj_VVLP, jj_WZ_sig_frac_CMS_jj_VVLP);

jj_WZ_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWLP, jj_WZ_sig_gsigma_CMS_jj_WWLP);
jjCBSigWZ_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWLP, jj_WZ_sig_sigma_CMS_jj_WWLP, jj_WZ_sig_alpha_CMS_jj_WWLP, jj_WZ_sig_n_CMS_jj_WWLP);
WZ_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigWZ_CMS_jj_WWLP, jjCBSigWZ_CMS_jj_WWLP, jj_WZ_sig_frac_CMS_jj_WWLP);

jj_WZ_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZLP, jj_WZ_sig_gsigma_CMS_jj_WZLP);
jjCBSigWZ_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZLP, jj_WZ_sig_sigma_CMS_jj_WZLP, jj_WZ_sig_alpha_CMS_jj_WZLP, jj_WZ_sig_n_CMS_jj_WZLP);
WZ_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigWZ_CMS_jj_WZLP, jjCBSigWZ_CMS_jj_WZLP, jj_WZ_sig_frac_CMS_jj_WZLP);

jj_WZ_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZLP, jj_WZ_sig_gsigma_CMS_jj_ZZLP);
jjCBSigWZ_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZLP, jj_WZ_sig_sigma_CMS_jj_ZZLP, jj_WZ_sig_alpha_CMS_jj_ZZLP, jj_WZ_sig_n_CMS_jj_ZZLP);
WZ_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigWZ_CMS_jj_ZZLP, jjCBSigWZ_CMS_jj_ZZLP, jj_WZ_sig_frac_CMS_jj_ZZLP);





jj_WZ_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigWZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_sigma_CMS_jj_VVNP, jj_WZ_sig_alpha_CMS_jj_VVNP, jj_WZ_sig_n_CMS_jj_VVNP);
WZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigWZ_CMS_jj_VVNP, jjCBSigWZ_CMS_jj_VVNP, jj_WZ_sig_frac_CMS_jj_VVNP);

jj_WZ_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWNP, jj_WZ_sig_gsigma_CMS_jj_WWNP);
jjCBSigWZ_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WWNP, jj_WZ_sig_sigma_CMS_jj_WWNP, jj_WZ_sig_alpha_CMS_jj_WWNP, jj_WZ_sig_n_CMS_jj_WWNP);
WZ_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigWZ_CMS_jj_WWNP, jjCBSigWZ_CMS_jj_WWNP, jj_WZ_sig_frac_CMS_jj_WWNP);

jj_WZ_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_WZNP[100, -1.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZNP, jj_WZ_sig_gsigma_CMS_jj_WZNP);
jjCBSigWZ_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_WZNP, jj_WZ_sig_sigma_CMS_jj_WZNP, jj_WZ_sig_alpha_CMS_jj_WZNP, jj_WZ_sig_n_CMS_jj_WZNP);
WZ_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigWZ_CMS_jj_WZNP, jjCBSigWZ_CMS_jj_WZNP, jj_WZ_sig_frac_CMS_jj_WZNP);

jj_WZ_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZNP, jj_WZ_sig_gsigma_CMS_jj_ZZNP);
jjCBSigWZ_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_ZZNP, jj_WZ_sig_sigma_CMS_jj_ZZNP, jj_WZ_sig_alpha_CMS_jj_ZZNP, jj_WZ_sig_n_CMS_jj_ZZNP);
WZ_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigWZ_CMS_jj_ZZNP, jjCBSigWZ_CMS_jj_ZZNP, jj_WZ_sig_frac_CMS_jj_ZZNP);




jj_WZ_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigWZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_sigma_CMS_jj_qVHP, jj_WZ_sig_alpha_CMS_jj_qVHP, jj_WZ_sig_n_CMS_jj_qVHP);
WZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigWZ_CMS_jj_qVHP, jjCBSigWZ_CMS_jj_qVHP, jj_WZ_sig_frac_CMS_jj_qVHP);

jj_WZ_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWHP, jj_WZ_sig_gsigma_CMS_jj_qWHP);
jjCBSigWZ_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWHP, jj_WZ_sig_sigma_CMS_jj_qWHP, jj_WZ_sig_alpha_CMS_jj_qWHP, jj_WZ_sig_n_CMS_jj_qWHP);
WZ_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigWZ_CMS_jj_qWHP, jjCBSigWZ_CMS_jj_qWHP, jj_WZ_sig_frac_CMS_jj_qWHP);

jj_WZ_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZHP, jj_WZ_sig_gsigma_CMS_jj_qZHP);
jjCBSigWZ_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZHP, jj_WZ_sig_sigma_CMS_jj_qZHP, jj_WZ_sig_alpha_CMS_jj_qZHP, jj_WZ_sig_n_CMS_jj_qZHP);
WZ_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigWZ_CMS_jj_qZHP, jjCBSigWZ_CMS_jj_qZHP, jj_WZ_sig_frac_CMS_jj_qZHP);



jj_WZ_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigWZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_sigma_CMS_jj_qVLP, jj_WZ_sig_alpha_CMS_jj_qVLP, jj_WZ_sig_n_CMS_jj_qVLP);
WZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigWZ_CMS_jj_qVLP, jjCBSigWZ_CMS_jj_qVLP, jj_WZ_sig_frac_CMS_jj_qVLP);

jj_WZ_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWLP, jj_WZ_sig_gsigma_CMS_jj_qWLP);
jjCBSigWZ_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWLP, jj_WZ_sig_sigma_CMS_jj_qWLP, jj_WZ_sig_alpha_CMS_jj_qWLP, jj_WZ_sig_n_CMS_jj_qWLP);
WZ_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigWZ_CMS_jj_qWLP, jjCBSigWZ_CMS_jj_qWLP, jj_WZ_sig_frac_CMS_jj_qWLP);

jj_WZ_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZLP, jj_WZ_sig_gsigma_CMS_jj_qZLP);
jjCBSigWZ_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZLP, jj_WZ_sig_sigma_CMS_jj_qZLP, jj_WZ_sig_alpha_CMS_jj_qZLP, jj_WZ_sig_n_CMS_jj_qZLP);
WZ_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigWZ_CMS_jj_qZLP, jjCBSigWZ_CMS_jj_qZLP, jj_WZ_sig_frac_CMS_jj_qZLP);




jj_WZ_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigWZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_sigma_CMS_jj_qVNP, jj_WZ_sig_alpha_CMS_jj_qVNP, jj_WZ_sig_n_CMS_jj_qVNP);
WZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigWZ_CMS_jj_qVNP, jjCBSigWZ_CMS_jj_qVNP, jj_WZ_sig_frac_CMS_jj_qVNP);

jj_WZ_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWNP, jj_WZ_sig_gsigma_CMS_jj_qWNP);
jjCBSigWZ_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qWNP, jj_WZ_sig_sigma_CMS_jj_qWNP, jj_WZ_sig_alpha_CMS_jj_qWNP, jj_WZ_sig_n_CMS_jj_qWNP);
WZ_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigWZ_CMS_jj_qWNP, jjCBSigWZ_CMS_jj_qWNP, jj_WZ_sig_frac_CMS_jj_qWNP);

jj_WZ_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4000.0];
jj_WZ_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigWZ_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZNP, jj_WZ_sig_gsigma_CMS_jj_qZNP);
jjCBSigWZ_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qZNP, jj_WZ_sig_sigma_CMS_jj_qZNP, jj_WZ_sig_alpha_CMS_jj_qZNP, jj_WZ_sig_n_CMS_jj_qZNP);
WZ_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigWZ_CMS_jj_qZNP, jjCBSigWZ_CMS_jj_qZNP, jj_WZ_sig_frac_CMS_jj_qZNP);








jj_qW_sig_m0[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma[100, 0.0, 1000.0];
jj_qW_sig_alpha[0.8, 0.5, 3]; 
jj_qW_sig_n[13.0, 0.5, 10]; 
jj_qW_sig_gsigma[100, 0.0, 1000.0];
jj_qW_sig_frac[0.5, 0.0, 1.0];

jjGaussSigqW = Gaussian(mgg13TeV, jj_qW_sig_m0, jj_qW_sig_gsigma);
jjCBSigqW    = CBShape(mgg13TeV, jj_qW_sig_m0, jj_qW_sig_sigma, jj_qW_sig_alpha, jj_qW_sig_n);
qW_jj      = AddPdf(jjGaussSigqW, jjCBSigqW, jj_qW_sig_frac);

jj_qW_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_gsigma_CMS_jj_VVHP);
jjCBSigqW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_sigma_CMS_jj_VVHP, jj_qW_sig_alpha_CMS_jj_VVHP, jj_qW_sig_n_CMS_jj_VVHP);
qW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigqW_CMS_jj_VVHP, jjCBSigqW_CMS_jj_VVHP, jj_qW_sig_frac_CMS_jj_VVHP);

jj_qW_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWHP, jj_qW_sig_gsigma_CMS_jj_WWHP);
jjCBSigqW_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWHP, jj_qW_sig_sigma_CMS_jj_WWHP, jj_qW_sig_alpha_CMS_jj_WWHP, jj_qW_sig_n_CMS_jj_WWHP);
qW_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigqW_CMS_jj_WWHP, jjCBSigqW_CMS_jj_WWHP, jj_qW_sig_frac_CMS_jj_WWHP);

jj_qW_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZHP, jj_qW_sig_gsigma_CMS_jj_WZHP);
jjCBSigqW_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZHP, jj_qW_sig_sigma_CMS_jj_WZHP, jj_qW_sig_alpha_CMS_jj_WZHP, jj_qW_sig_n_CMS_jj_WZHP);
qW_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigqW_CMS_jj_WZHP, jjCBSigqW_CMS_jj_WZHP, jj_qW_sig_frac_CMS_jj_WZHP);

jj_qW_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZHP, jj_qW_sig_gsigma_CMS_jj_ZZHP);
jjCBSigqW_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZHP, jj_qW_sig_sigma_CMS_jj_ZZHP, jj_qW_sig_alpha_CMS_jj_ZZHP, jj_qW_sig_n_CMS_jj_ZZHP);
qW_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigqW_CMS_jj_ZZHP, jjCBSigqW_CMS_jj_ZZHP, jj_qW_sig_frac_CMS_jj_ZZHP);





jj_qW_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_gsigma_CMS_jj_VVLP);
jjCBSigqW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_sigma_CMS_jj_VVLP, jj_qW_sig_alpha_CMS_jj_VVLP, jj_qW_sig_n_CMS_jj_VVLP);
qW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigqW_CMS_jj_VVLP, jjCBSigqW_CMS_jj_VVLP, jj_qW_sig_frac_CMS_jj_VVLP);

jj_qW_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWLP, jj_qW_sig_gsigma_CMS_jj_WWLP);
jjCBSigqW_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWLP, jj_qW_sig_sigma_CMS_jj_WWLP, jj_qW_sig_alpha_CMS_jj_WWLP, jj_qW_sig_n_CMS_jj_WWLP);
qW_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigqW_CMS_jj_WWLP, jjCBSigqW_CMS_jj_WWLP, jj_qW_sig_frac_CMS_jj_WWLP);

jj_qW_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZLP, jj_qW_sig_gsigma_CMS_jj_WZLP);
jjCBSigqW_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZLP, jj_qW_sig_sigma_CMS_jj_WZLP, jj_qW_sig_alpha_CMS_jj_WZLP, jj_qW_sig_n_CMS_jj_WZLP);
qW_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigqW_CMS_jj_WZLP, jjCBSigqW_CMS_jj_WZLP, jj_qW_sig_frac_CMS_jj_WZLP);


jj_qW_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZLP, jj_qW_sig_gsigma_CMS_jj_ZZLP);
jjCBSigqW_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZLP, jj_qW_sig_sigma_CMS_jj_ZZLP, jj_qW_sig_alpha_CMS_jj_ZZLP, jj_qW_sig_n_CMS_jj_ZZLP);
qW_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigqW_CMS_jj_ZZLP, jjCBSigqW_CMS_jj_ZZLP, jj_qW_sig_frac_CMS_jj_ZZLP);



jj_qW_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_gsigma_CMS_jj_VVNP);
jjCBSigqW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_sigma_CMS_jj_VVNP, jj_qW_sig_alpha_CMS_jj_VVNP, jj_qW_sig_n_CMS_jj_VVNP);
qW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigqW_CMS_jj_VVNP, jjCBSigqW_CMS_jj_VVNP, jj_qW_sig_frac_CMS_jj_VVNP);

jj_qW_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWNP, jj_qW_sig_gsigma_CMS_jj_WWNP);
jjCBSigqW_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WWNP, jj_qW_sig_sigma_CMS_jj_WWNP, jj_qW_sig_alpha_CMS_jj_WWNP, jj_qW_sig_n_CMS_jj_WWNP);
qW_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigqW_CMS_jj_WWNP, jjCBSigqW_CMS_jj_WWNP, jj_qW_sig_frac_CMS_jj_WWNP);

jj_qW_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZNP, jj_qW_sig_gsigma_CMS_jj_WZNP);
jjCBSigqW_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_WZNP, jj_qW_sig_sigma_CMS_jj_WZNP, jj_qW_sig_alpha_CMS_jj_WZNP, jj_qW_sig_n_CMS_jj_WZNP);
qW_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigqW_CMS_jj_WZNP, jjCBSigqW_CMS_jj_WZNP, jj_qW_sig_frac_CMS_jj_WZNP);

jj_qW_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZNP, jj_qW_sig_gsigma_CMS_jj_ZZNP);
jjCBSigqW_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_ZZNP, jj_qW_sig_sigma_CMS_jj_ZZNP, jj_qW_sig_alpha_CMS_jj_ZZNP, jj_qW_sig_n_CMS_jj_ZZNP);
qW_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigqW_CMS_jj_ZZNP, jjCBSigqW_CMS_jj_ZZNP, jj_qW_sig_frac_CMS_jj_ZZNP);



jj_qW_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_gsigma_CMS_jj_qVHP);
jjCBSigqW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_sigma_CMS_jj_qVHP, jj_qW_sig_alpha_CMS_jj_qVHP, jj_qW_sig_n_CMS_jj_qVHP);
qW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigqW_CMS_jj_qVHP, jjCBSigqW_CMS_jj_qVHP, jj_qW_sig_frac_CMS_jj_qVHP);

jj_qW_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWHP, jj_qW_sig_gsigma_CMS_jj_qWHP);
jjCBSigqW_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWHP, jj_qW_sig_sigma_CMS_jj_qWHP, jj_qW_sig_alpha_CMS_jj_qWHP, jj_qW_sig_n_CMS_jj_qWHP);
qW_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigqW_CMS_jj_qWHP, jjCBSigqW_CMS_jj_qWHP, jj_qW_sig_frac_CMS_jj_qWHP);

jj_qW_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZHP, jj_qW_sig_gsigma_CMS_jj_qZHP);
jjCBSigqW_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZHP, jj_qW_sig_sigma_CMS_jj_qZHP, jj_qW_sig_alpha_CMS_jj_qZHP, jj_qW_sig_n_CMS_jj_qZHP);
qW_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigqW_CMS_jj_qZHP, jjCBSigqW_CMS_jj_qZHP, jj_qW_sig_frac_CMS_jj_qZHP);



jj_qW_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_gsigma_CMS_jj_qVLP);
jjCBSigqW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_sigma_CMS_jj_qVLP, jj_qW_sig_alpha_CMS_jj_qVLP, jj_qW_sig_n_CMS_jj_qVLP);
qW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigqW_CMS_jj_qVLP, jjCBSigqW_CMS_jj_qVLP, jj_qW_sig_frac_CMS_jj_qVLP);

jj_qW_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWLP, jj_qW_sig_gsigma_CMS_jj_qWLP);
jjCBSigqW_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWLP, jj_qW_sig_sigma_CMS_jj_qWLP, jj_qW_sig_alpha_CMS_jj_qWLP, jj_qW_sig_n_CMS_jj_qWLP);
qW_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigqW_CMS_jj_qWLP, jjCBSigqW_CMS_jj_qWLP, jj_qW_sig_frac_CMS_jj_qWLP);


jj_qW_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZLP, jj_qW_sig_gsigma_CMS_jj_qZLP);
jjCBSigqW_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZLP, jj_qW_sig_sigma_CMS_jj_qZLP, jj_qW_sig_alpha_CMS_jj_qZLP, jj_qW_sig_n_CMS_jj_qZLP);
qW_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigqW_CMS_jj_qZLP, jjCBSigqW_CMS_jj_qZLP, jj_qW_sig_frac_CMS_jj_qZLP);





jj_qW_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_gsigma_CMS_jj_qVNP);
jjCBSigqW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_sigma_CMS_jj_qVNP, jj_qW_sig_alpha_CMS_jj_qVNP, jj_qW_sig_n_CMS_jj_qVNP);
qW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigqW_CMS_jj_qVNP, jjCBSigqW_CMS_jj_qVNP, jj_qW_sig_frac_CMS_jj_qVNP);

jj_qW_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWNP, jj_qW_sig_gsigma_CMS_jj_qWNP);
jjCBSigqW_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qWNP, jj_qW_sig_sigma_CMS_jj_qWNP, jj_qW_sig_alpha_CMS_jj_qWNP, jj_qW_sig_n_CMS_jj_qWNP);
qW_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigqW_CMS_jj_qWNP, jjCBSigqW_CMS_jj_qWNP, jj_qW_sig_frac_CMS_jj_qWNP);

jj_qW_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigqW_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZNP, jj_qW_sig_gsigma_CMS_jj_qZNP);
jjCBSigqW_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qZNP, jj_qW_sig_sigma_CMS_jj_qZNP, jj_qW_sig_alpha_CMS_jj_qZNP, jj_qW_sig_n_CMS_jj_qZNP);
qW_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigqW_CMS_jj_qZNP, jjCBSigqW_CMS_jj_qZNP, jj_qW_sig_frac_CMS_jj_qZNP);







jj_qZ_sig_m0[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma[100, 0.0, 1000.0];
jj_qZ_sig_alpha[0.8, 0.5, 3]; 
jj_qZ_sig_n[13.0, 0.5, 10]; 
jj_qZ_sig_gsigma[100, 0.0, 1000.0];
jj_qZ_sig_frac[0.5, 0.0, 1.0];

jjGaussSigqZ = Gaussian(mgg13TeV, jj_qZ_sig_m0, jj_qZ_sig_gsigma);
jjCBSigqZ    = CBShape(mgg13TeV, jj_qZ_sig_m0, jj_qZ_sig_sigma, jj_qZ_sig_alpha, jj_qZ_sig_n);
qZ_jj      = AddPdf(jjGaussSigqZ, jjCBSigqZ, jj_qZ_sig_frac);

jj_qZ_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];


jjGaussSigqZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigqZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_sigma_CMS_jj_VVHP, jj_qZ_sig_alpha_CMS_jj_VVHP, jj_qZ_sig_n_CMS_jj_VVHP);
qZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigqZ_CMS_jj_VVHP, jjCBSigqZ_CMS_jj_VVHP, jj_qZ_sig_frac_CMS_jj_VVHP);

jj_qZ_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];


jjGaussSigqZ_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWHP, jj_qZ_sig_gsigma_CMS_jj_WWHP);
jjCBSigqZ_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWHP, jj_qZ_sig_sigma_CMS_jj_WWHP, jj_qZ_sig_alpha_CMS_jj_WWHP, jj_qZ_sig_n_CMS_jj_WWHP);
qZ_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigqZ_CMS_jj_WWHP, jjCBSigqZ_CMS_jj_WWHP, jj_qZ_sig_frac_CMS_jj_WWHP);

jj_qZ_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];


jjGaussSigqZ_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZHP, jj_qZ_sig_gsigma_CMS_jj_WZHP);
jjCBSigqZ_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZHP, jj_qZ_sig_sigma_CMS_jj_WZHP, jj_qZ_sig_alpha_CMS_jj_WZHP, jj_qZ_sig_n_CMS_jj_WZHP);
qZ_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigqZ_CMS_jj_WZHP, jjCBSigqZ_CMS_jj_WZHP, jj_qZ_sig_frac_CMS_jj_WZHP);

jj_qZ_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];


jjGaussSigqZ_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZHP, jj_qZ_sig_gsigma_CMS_jj_ZZHP);
jjCBSigqZ_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZHP, jj_qZ_sig_sigma_CMS_jj_ZZHP, jj_qZ_sig_alpha_CMS_jj_ZZHP, jj_qZ_sig_n_CMS_jj_ZZHP);
qZ_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigqZ_CMS_jj_ZZHP, jjCBSigqZ_CMS_jj_ZZHP, jj_qZ_sig_frac_CMS_jj_ZZHP);




jj_qZ_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigqZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_sigma_CMS_jj_VVLP, jj_qZ_sig_alpha_CMS_jj_VVLP, jj_qZ_sig_n_CMS_jj_VVLP);
qZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigqZ_CMS_jj_VVLP, jjCBSigqZ_CMS_jj_VVLP, jj_qZ_sig_frac_CMS_jj_VVLP);

jj_qZ_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWLP, jj_qZ_sig_gsigma_CMS_jj_WWLP);
jjCBSigqZ_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWLP, jj_qZ_sig_sigma_CMS_jj_WWLP, jj_qZ_sig_alpha_CMS_jj_WWLP, jj_qZ_sig_n_CMS_jj_WWLP);
qZ_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigqZ_CMS_jj_WWLP, jjCBSigqZ_CMS_jj_WWLP, jj_qZ_sig_frac_CMS_jj_WWLP);


jj_qZ_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZLP, jj_qZ_sig_gsigma_CMS_jj_WZLP);
jjCBSigqZ_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZLP, jj_qZ_sig_sigma_CMS_jj_WZLP, jj_qZ_sig_alpha_CMS_jj_WZLP, jj_qZ_sig_n_CMS_jj_WZLP);
qZ_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigqZ_CMS_jj_WZLP, jjCBSigqZ_CMS_jj_WZLP, jj_qZ_sig_frac_CMS_jj_WZLP);


jj_qZ_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZLP, jj_qZ_sig_gsigma_CMS_jj_ZZLP);
jjCBSigqZ_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZLP, jj_qZ_sig_sigma_CMS_jj_ZZLP, jj_qZ_sig_alpha_CMS_jj_ZZLP, jj_qZ_sig_n_CMS_jj_ZZLP);
qZ_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigqZ_CMS_jj_ZZLP, jjCBSigqZ_CMS_jj_ZZLP, jj_qZ_sig_frac_CMS_jj_ZZLP);






jj_qZ_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigqZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_sigma_CMS_jj_VVNP, jj_qZ_sig_alpha_CMS_jj_VVNP, jj_qZ_sig_n_CMS_jj_VVNP);
qZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigqZ_CMS_jj_VVNP, jjCBSigqZ_CMS_jj_VVNP, jj_qZ_sig_frac_CMS_jj_VVNP);

jj_qZ_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWNP, jj_qZ_sig_gsigma_CMS_jj_WWNP);
jjCBSigqZ_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WWNP, jj_qZ_sig_sigma_CMS_jj_WWNP, jj_qZ_sig_alpha_CMS_jj_WWNP, jj_qZ_sig_n_CMS_jj_WWNP);
qZ_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigqZ_CMS_jj_WWNP, jjCBSigqZ_CMS_jj_WWNP, jj_qZ_sig_frac_CMS_jj_WWNP);

jj_qZ_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZNP, jj_qZ_sig_gsigma_CMS_jj_WZNP);
jjCBSigqZ_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_WZNP, jj_qZ_sig_sigma_CMS_jj_WZNP, jj_qZ_sig_alpha_CMS_jj_WZNP, jj_qZ_sig_n_CMS_jj_WZNP);
qZ_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigqZ_CMS_jj_WZNP, jjCBSigqZ_CMS_jj_WZNP, jj_qZ_sig_frac_CMS_jj_WZNP);

jj_qZ_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZNP, jj_qZ_sig_gsigma_CMS_jj_ZZNP);
jjCBSigqZ_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_ZZNP, jj_qZ_sig_sigma_CMS_jj_ZZNP, jj_qZ_sig_alpha_CMS_jj_ZZNP, jj_qZ_sig_n_CMS_jj_ZZNP);
qZ_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigqZ_CMS_jj_ZZNP, jjCBSigqZ_CMS_jj_ZZNP, jj_qZ_sig_frac_CMS_jj_ZZNP);




jj_qZ_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigqZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_sigma_CMS_jj_qVHP, jj_qZ_sig_alpha_CMS_jj_qVHP, jj_qZ_sig_n_CMS_jj_qVHP);
qZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigqZ_CMS_jj_qVHP, jjCBSigqZ_CMS_jj_qVHP, jj_qZ_sig_frac_CMS_jj_qVHP);

jj_qZ_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWHP, jj_qZ_sig_gsigma_CMS_jj_qWHP);
jjCBSigqZ_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWHP, jj_qZ_sig_sigma_CMS_jj_qWHP, jj_qZ_sig_alpha_CMS_jj_qWHP, jj_qZ_sig_n_CMS_jj_qWHP);
qZ_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigqZ_CMS_jj_qWHP, jjCBSigqZ_CMS_jj_qWHP, jj_qZ_sig_frac_CMS_jj_qWHP);

jj_qZ_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZHP, jj_qZ_sig_gsigma_CMS_jj_qZHP);
jjCBSigqZ_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZHP, jj_qZ_sig_sigma_CMS_jj_qZHP, jj_qZ_sig_alpha_CMS_jj_qZHP, jj_qZ_sig_n_CMS_jj_qZHP);
qZ_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigqZ_CMS_jj_qZHP, jjCBSigqZ_CMS_jj_qZHP, jj_qZ_sig_frac_CMS_jj_qZHP);



jj_qZ_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigqZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_sigma_CMS_jj_qVLP, jj_qZ_sig_alpha_CMS_jj_qVLP, jj_qZ_sig_n_CMS_jj_qVLP);
qZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigqZ_CMS_jj_qVLP, jjCBSigqZ_CMS_jj_qVLP, jj_qZ_sig_frac_CMS_jj_qVLP);

jj_qZ_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWLP, jj_qZ_sig_gsigma_CMS_jj_qWLP);
jjCBSigqZ_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWLP, jj_qZ_sig_sigma_CMS_jj_qWLP, jj_qZ_sig_alpha_CMS_jj_qWLP, jj_qZ_sig_n_CMS_jj_qWLP);
qZ_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigqZ_CMS_jj_qWLP, jjCBSigqZ_CMS_jj_qWLP, jj_qZ_sig_frac_CMS_jj_qWLP);

jj_qZ_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZLP, jj_qZ_sig_gsigma_CMS_jj_qZLP);
jjCBSigqZ_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZLP, jj_qZ_sig_sigma_CMS_jj_qZLP, jj_qZ_sig_alpha_CMS_jj_qZLP, jj_qZ_sig_n_CMS_jj_qZLP);
qZ_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigqZ_CMS_jj_qZLP, jjCBSigqZ_CMS_jj_qZLP, jj_qZ_sig_frac_CMS_jj_qZLP);




jj_qZ_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigqZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_sigma_CMS_jj_qVNP, jj_qZ_sig_alpha_CMS_jj_qVNP, jj_qZ_sig_n_CMS_jj_qVNP);
qZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigqZ_CMS_jj_qVNP, jjCBSigqZ_CMS_jj_qVNP, jj_qZ_sig_frac_CMS_jj_qVNP);

jj_qZ_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWNP, jj_qZ_sig_gsigma_CMS_jj_qWNP);
jjCBSigqZ_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qWNP, jj_qZ_sig_sigma_CMS_jj_qWNP, jj_qZ_sig_alpha_CMS_jj_qWNP, jj_qZ_sig_n_CMS_jj_qWNP);
qZ_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigqZ_CMS_jj_qWNP, jjCBSigqZ_CMS_jj_qWNP, jj_qZ_sig_frac_CMS_jj_qWNP);

jj_qZ_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigqZ_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZNP, jj_qZ_sig_gsigma_CMS_jj_qZNP);
jjCBSigqZ_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qZNP, jj_qZ_sig_sigma_CMS_jj_qZNP, jj_qZ_sig_alpha_CMS_jj_qZNP, jj_qZ_sig_n_CMS_jj_qZNP);
qZ_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigqZ_CMS_jj_qZNP, jjCBSigqZ_CMS_jj_qZNP, jj_qZ_sig_frac_CMS_jj_qZNP);


jj_BulkZZ_sig_m0[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha[0.8, 0.5, 3]; 
jj_BulkZZ_sig_n[13.0, 0.5, 10]; 
jj_BulkZZ_sig_gsigma[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac[0.5, 0.0, 1.0];

jjGaussSigBulkZZ = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_gsigma);
jjCBSigBulkZZ    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_sigma, jj_BulkZZ_sig_alpha, jj_BulkZZ_sig_n);
BulkZZ_jj      = AddPdf(jjGaussSigBulkZZ, jjCBSigBulkZZ, jj_BulkZZ_sig_frac);

jj_BulkZZ_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];


jjGaussSigBulkZZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkZZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_sigma_CMS_jj_VVHP, jj_BulkZZ_sig_alpha_CMS_jj_VVHP, jj_BulkZZ_sig_n_CMS_jj_VVHP);
BulkZZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVHP, jjCBSigBulkZZ_CMS_jj_VVHP, jj_BulkZZ_sig_frac_CMS_jj_VVHP);

jj_BulkZZ_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 5000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 10.0]; 
jj_BulkZZ_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WWHP[100, 0.0, 10000.0];
jj_BulkZZ_sig_frac_CMS_jj_WWHP[0.5, 0.00, 1.0];

jjGaussSigBulkZZ_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWHP, jj_BulkZZ_sig_gsigma_CMS_jj_WWHP);
jjCBSigBulkZZ_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWHP, jj_BulkZZ_sig_sigma_CMS_jj_WWHP, jj_BulkZZ_sig_alpha_CMS_jj_WWHP, jj_BulkZZ_sig_n_CMS_jj_WWHP);
BulkZZ_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WWHP, jjCBSigBulkZZ_CMS_jj_WWHP, jj_BulkZZ_sig_frac_CMS_jj_WWHP);

jj_BulkZZ_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 5000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WZHP[50, 0.0, 500.0];
jj_BulkZZ_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WZHP[500, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_WZHP[0.5, 0.30, 1.0];


jjGaussSigBulkZZ_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZHP, jj_BulkZZ_sig_gsigma_CMS_jj_WZHP);
jjCBSigBulkZZ_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZHP, jj_BulkZZ_sig_sigma_CMS_jj_WZHP, jj_BulkZZ_sig_alpha_CMS_jj_WZHP, jj_BulkZZ_sig_n_CMS_jj_WZHP);
BulkZZ_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WZHP, jjCBSigBulkZZ_CMS_jj_WZHP, jj_BulkZZ_sig_frac_CMS_jj_WZHP);

jj_BulkZZ_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];


jjGaussSigBulkZZ_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZHP, jj_BulkZZ_sig_gsigma_CMS_jj_ZZHP);
jjCBSigBulkZZ_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZHP, jj_BulkZZ_sig_sigma_CMS_jj_ZZHP, jj_BulkZZ_sig_alpha_CMS_jj_ZZHP, jj_BulkZZ_sig_n_CMS_jj_ZZHP);
BulkZZ_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_ZZHP, jjCBSigBulkZZ_CMS_jj_ZZHP, jj_BulkZZ_sig_frac_CMS_jj_ZZHP);



jj_BulkZZ_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVLP[1000, 0.0, 10000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkZZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_sigma_CMS_jj_VVLP, jj_BulkZZ_sig_alpha_CMS_jj_VVLP, jj_BulkZZ_sig_n_CMS_jj_VVLP);
BulkZZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVLP, jjCBSigBulkZZ_CMS_jj_VVLP, jj_BulkZZ_sig_frac_CMS_jj_VVLP);

jj_BulkZZ_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 5000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WWLP[50, 0.0, 500.0];
jj_BulkZZ_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WWLP[500, 100.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_WWLP[0.5, 0.3, 1.0];



jjGaussSigBulkZZ_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWLP, jj_BulkZZ_sig_gsigma_CMS_jj_WWLP);
jjCBSigBulkZZ_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWLP, jj_BulkZZ_sig_sigma_CMS_jj_WWLP, jj_BulkZZ_sig_alpha_CMS_jj_WWLP, jj_BulkZZ_sig_n_CMS_jj_WWLP);
BulkZZ_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WWLP, jjCBSigBulkZZ_CMS_jj_WWLP, jj_BulkZZ_sig_frac_CMS_jj_WWLP);

jj_BulkZZ_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZLP, jj_BulkZZ_sig_gsigma_CMS_jj_WZLP);
jjCBSigBulkZZ_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZLP, jj_BulkZZ_sig_sigma_CMS_jj_WZLP, jj_BulkZZ_sig_alpha_CMS_jj_WZLP, jj_BulkZZ_sig_n_CMS_jj_WZLP);
BulkZZ_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WZLP, jjCBSigBulkZZ_CMS_jj_WZLP, jj_BulkZZ_sig_frac_CMS_jj_WZLP);

jj_BulkZZ_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZLP, jj_BulkZZ_sig_gsigma_CMS_jj_ZZLP);
jjCBSigBulkZZ_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZLP, jj_BulkZZ_sig_sigma_CMS_jj_ZZLP, jj_BulkZZ_sig_alpha_CMS_jj_ZZLP, jj_BulkZZ_sig_n_CMS_jj_ZZLP);
BulkZZ_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_ZZLP, jjCBSigBulkZZ_CMS_jj_ZZLP, jj_BulkZZ_sig_frac_CMS_jj_ZZLP);




jj_BulkZZ_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkZZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_sigma_CMS_jj_VVNP, jj_BulkZZ_sig_alpha_CMS_jj_VVNP, jj_BulkZZ_sig_n_CMS_jj_VVNP);
BulkZZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVNP, jjCBSigBulkZZ_CMS_jj_VVNP, jj_BulkZZ_sig_frac_CMS_jj_VVNP);

jj_BulkZZ_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWNP, jj_BulkZZ_sig_gsigma_CMS_jj_WWNP);
jjCBSigBulkZZ_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WWNP, jj_BulkZZ_sig_sigma_CMS_jj_WWNP, jj_BulkZZ_sig_alpha_CMS_jj_WWNP, jj_BulkZZ_sig_n_CMS_jj_WWNP);
BulkZZ_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WWNP, jjCBSigBulkZZ_CMS_jj_WWNP, jj_BulkZZ_sig_frac_CMS_jj_WWNP);

jj_BulkZZ_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZNP, jj_BulkZZ_sig_gsigma_CMS_jj_WZNP);
jjCBSigBulkZZ_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_WZNP, jj_BulkZZ_sig_sigma_CMS_jj_WZNP, jj_BulkZZ_sig_alpha_CMS_jj_WZNP, jj_BulkZZ_sig_n_CMS_jj_WZNP);
BulkZZ_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_WZNP, jjCBSigBulkZZ_CMS_jj_WZNP, jj_BulkZZ_sig_frac_CMS_jj_WZNP);

jj_BulkZZ_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZNP, jj_BulkZZ_sig_gsigma_CMS_jj_ZZNP);
jjCBSigBulkZZ_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_ZZNP, jj_BulkZZ_sig_sigma_CMS_jj_ZZNP, jj_BulkZZ_sig_alpha_CMS_jj_ZZNP, jj_BulkZZ_sig_n_CMS_jj_ZZNP);
BulkZZ_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_ZZNP, jjCBSigBulkZZ_CMS_jj_ZZNP, jj_BulkZZ_sig_frac_CMS_jj_ZZNP);





jj_BulkZZ_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkZZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_sigma_CMS_jj_qVHP, jj_BulkZZ_sig_alpha_CMS_jj_qVHP, jj_BulkZZ_sig_n_CMS_jj_qVHP);
BulkZZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVHP, jjCBSigBulkZZ_CMS_jj_qVHP, jj_BulkZZ_sig_frac_CMS_jj_qVHP);

jj_BulkZZ_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWHP, jj_BulkZZ_sig_gsigma_CMS_jj_qWHP);
jjCBSigBulkZZ_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWHP, jj_BulkZZ_sig_sigma_CMS_jj_qWHP, jj_BulkZZ_sig_alpha_CMS_jj_qWHP, jj_BulkZZ_sig_n_CMS_jj_qWHP);
BulkZZ_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qWHP, jjCBSigBulkZZ_CMS_jj_qWHP, jj_BulkZZ_sig_frac_CMS_jj_qWHP);

jj_BulkZZ_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZHP, jj_BulkZZ_sig_gsigma_CMS_jj_qZHP);
jjCBSigBulkZZ_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZHP, jj_BulkZZ_sig_sigma_CMS_jj_qZHP, jj_BulkZZ_sig_alpha_CMS_jj_qZHP, jj_BulkZZ_sig_n_CMS_jj_qZHP);
BulkZZ_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qZHP, jjCBSigBulkZZ_CMS_jj_qZHP, jj_BulkZZ_sig_frac_CMS_jj_qZHP);



jj_BulkZZ_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkZZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_sigma_CMS_jj_qVLP, jj_BulkZZ_sig_alpha_CMS_jj_qVLP, jj_BulkZZ_sig_n_CMS_jj_qVLP);
BulkZZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVLP, jjCBSigBulkZZ_CMS_jj_qVLP, jj_BulkZZ_sig_frac_CMS_jj_qVLP);

jj_BulkZZ_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWLP, jj_BulkZZ_sig_gsigma_CMS_jj_qWLP);
jjCBSigBulkZZ_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWLP, jj_BulkZZ_sig_sigma_CMS_jj_qWLP, jj_BulkZZ_sig_alpha_CMS_jj_qWLP, jj_BulkZZ_sig_n_CMS_jj_qWLP);
BulkZZ_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qWLP, jjCBSigBulkZZ_CMS_jj_qWLP, jj_BulkZZ_sig_frac_CMS_jj_qWLP);

jj_BulkZZ_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZLP, jj_BulkZZ_sig_gsigma_CMS_jj_qZLP);
jjCBSigBulkZZ_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZLP, jj_BulkZZ_sig_sigma_CMS_jj_qZLP, jj_BulkZZ_sig_alpha_CMS_jj_qZLP, jj_BulkZZ_sig_n_CMS_jj_qZLP);
BulkZZ_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qZLP, jjCBSigBulkZZ_CMS_jj_qZLP, jj_BulkZZ_sig_frac_CMS_jj_qZLP);



jj_BulkZZ_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkZZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_sigma_CMS_jj_qVNP, jj_BulkZZ_sig_alpha_CMS_jj_qVNP, jj_BulkZZ_sig_n_CMS_jj_qVNP);
BulkZZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVNP, jjCBSigBulkZZ_CMS_jj_qVNP, jj_BulkZZ_sig_frac_CMS_jj_qVNP);

jj_BulkZZ_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWNP, jj_BulkZZ_sig_gsigma_CMS_jj_qWNP);
jjCBSigBulkZZ_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qWNP, jj_BulkZZ_sig_sigma_CMS_jj_qWNP, jj_BulkZZ_sig_alpha_CMS_jj_qWNP, jj_BulkZZ_sig_n_CMS_jj_qWNP);
BulkZZ_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qWNP, jjCBSigBulkZZ_CMS_jj_qWNP, jj_BulkZZ_sig_frac_CMS_jj_qWNP);

jj_BulkZZ_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4000.0];
jj_BulkZZ_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigBulkZZ_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZNP, jj_BulkZZ_sig_gsigma_CMS_jj_qZNP);
jjCBSigBulkZZ_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qZNP, jj_BulkZZ_sig_sigma_CMS_jj_qZNP, jj_BulkZZ_sig_alpha_CMS_jj_qZNP, jj_BulkZZ_sig_n_CMS_jj_qZNP);
BulkZZ_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qZNP, jjCBSigBulkZZ_CMS_jj_qZNP, jj_BulkZZ_sig_frac_CMS_jj_qZNP);








jj_BulkWW_sig_m0[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha[0.8, 0.5, 3]; 
jj_BulkWW_sig_n[13.0, 0.5, 10]; 
jj_BulkWW_sig_gsigma[100, 0.0, 1000.0];
jj_BulkWW_sig_frac[0.5, 0.0, 1.0];

jjGaussSigBulkWW = Gaussian(mgg13TeV, jj_BulkWW_sig_m0, jj_BulkWW_sig_gsigma);
jjCBSigBulkWW    = CBShape(mgg13TeV, jj_BulkWW_sig_m0, jj_BulkWW_sig_sigma, jj_BulkWW_sig_alpha, jj_BulkWW_sig_n);
BulkWW_jj      = AddPdf(jjGaussSigBulkWW, jjCBSigBulkWW, jj_BulkWW_sig_frac);

jj_BulkWW_sig_m0_CMS_jj_VVHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVHP[0.5, 0.0, 1.0];


jjGaussSigBulkWW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkWW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_sigma_CMS_jj_VVHP, jj_BulkWW_sig_alpha_CMS_jj_VVHP, jj_BulkWW_sig_n_CMS_jj_VVHP);
BulkWW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVHP, jjCBSigBulkWW_CMS_jj_VVHP, jj_BulkWW_sig_frac_CMS_jj_VVHP);

jj_BulkWW_sig_m0_CMS_jj_WWHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WWHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WWHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WWHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WWHP[0.5, 0.0, 1.0];


jjGaussSigBulkWW_CMS_jj_WWHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWHP, jj_BulkWW_sig_gsigma_CMS_jj_WWHP);
jjCBSigBulkWW_CMS_jj_WWHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWHP, jj_BulkWW_sig_sigma_CMS_jj_WWHP, jj_BulkWW_sig_alpha_CMS_jj_WWHP, jj_BulkWW_sig_n_CMS_jj_WWHP);
BulkWW_jj_CMS_jj_WWHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WWHP, jjCBSigBulkWW_CMS_jj_WWHP, jj_BulkWW_sig_frac_CMS_jj_WWHP);

jj_BulkWW_sig_m0_CMS_jj_WZHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WZHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WZHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WZHP[0.5, 0.0, 1.0];


jjGaussSigBulkWW_CMS_jj_WZHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZHP, jj_BulkWW_sig_gsigma_CMS_jj_WZHP);
jjCBSigBulkWW_CMS_jj_WZHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZHP, jj_BulkWW_sig_sigma_CMS_jj_WZHP, jj_BulkWW_sig_alpha_CMS_jj_WZHP, jj_BulkWW_sig_n_CMS_jj_WZHP);
BulkWW_jj_CMS_jj_WZHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WZHP, jjCBSigBulkWW_CMS_jj_WZHP, jj_BulkWW_sig_frac_CMS_jj_WZHP);

jj_BulkWW_sig_m0_CMS_jj_ZZHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_ZZHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_ZZHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_ZZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_ZZHP[0.5, 0.0, 1.0];


jjGaussSigBulkWW_CMS_jj_ZZHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZHP, jj_BulkWW_sig_gsigma_CMS_jj_ZZHP);
jjCBSigBulkWW_CMS_jj_ZZHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZHP, jj_BulkWW_sig_sigma_CMS_jj_ZZHP, jj_BulkWW_sig_alpha_CMS_jj_ZZHP, jj_BulkWW_sig_n_CMS_jj_ZZHP);
BulkWW_jj_CMS_jj_ZZHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_ZZHP, jjCBSigBulkWW_CMS_jj_ZZHP, jj_BulkWW_sig_frac_CMS_jj_ZZHP);





jj_BulkWW_sig_m0_CMS_jj_VVLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkWW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_sigma_CMS_jj_VVLP, jj_BulkWW_sig_alpha_CMS_jj_VVLP, jj_BulkWW_sig_n_CMS_jj_VVLP);
BulkWW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVLP, jjCBSigBulkWW_CMS_jj_VVLP, jj_BulkWW_sig_frac_CMS_jj_VVLP);

jj_BulkWW_sig_m0_CMS_jj_WWLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WWLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WWLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WWLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WWLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_WWLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWLP, jj_BulkWW_sig_gsigma_CMS_jj_WWLP);
jjCBSigBulkWW_CMS_jj_WWLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWLP, jj_BulkWW_sig_sigma_CMS_jj_WWLP, jj_BulkWW_sig_alpha_CMS_jj_WWLP, jj_BulkWW_sig_n_CMS_jj_WWLP);
BulkWW_jj_CMS_jj_WWLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WWLP, jjCBSigBulkWW_CMS_jj_WWLP, jj_BulkWW_sig_frac_CMS_jj_WWLP);

jj_BulkWW_sig_m0_CMS_jj_WZLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WZLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WZLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WZLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_WZLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZLP, jj_BulkWW_sig_gsigma_CMS_jj_WZLP);
jjCBSigBulkWW_CMS_jj_WZLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZLP, jj_BulkWW_sig_sigma_CMS_jj_WZLP, jj_BulkWW_sig_alpha_CMS_jj_WZLP, jj_BulkWW_sig_n_CMS_jj_WZLP);
BulkWW_jj_CMS_jj_WZLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WZLP, jjCBSigBulkWW_CMS_jj_WZLP, jj_BulkWW_sig_frac_CMS_jj_WZLP);

jj_BulkWW_sig_m0_CMS_jj_ZZLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_ZZLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_ZZLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_ZZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_ZZLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_ZZLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZLP, jj_BulkWW_sig_gsigma_CMS_jj_ZZLP);
jjCBSigBulkWW_CMS_jj_ZZLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZLP, jj_BulkWW_sig_sigma_CMS_jj_ZZLP, jj_BulkWW_sig_alpha_CMS_jj_ZZLP, jj_BulkWW_sig_n_CMS_jj_ZZLP);
BulkWW_jj_CMS_jj_ZZLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_ZZLP, jjCBSigBulkWW_CMS_jj_ZZLP, jj_BulkWW_sig_frac_CMS_jj_ZZLP);




jj_BulkWW_sig_m0_CMS_jj_VVNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkWW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_sigma_CMS_jj_VVNP, jj_BulkWW_sig_alpha_CMS_jj_VVNP, jj_BulkWW_sig_n_CMS_jj_VVNP);
BulkWW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVNP, jjCBSigBulkWW_CMS_jj_VVNP, jj_BulkWW_sig_frac_CMS_jj_VVNP);


jj_BulkWW_sig_m0_CMS_jj_WWNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WWNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WWNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WWNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WWNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_WWNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWNP, jj_BulkWW_sig_gsigma_CMS_jj_WWNP);
jjCBSigBulkWW_CMS_jj_WWNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WWNP, jj_BulkWW_sig_sigma_CMS_jj_WWNP, jj_BulkWW_sig_alpha_CMS_jj_WWNP, jj_BulkWW_sig_n_CMS_jj_WWNP);
BulkWW_jj_CMS_jj_WWNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WWNP, jjCBSigBulkWW_CMS_jj_WWNP, jj_BulkWW_sig_frac_CMS_jj_WWNP);

jj_BulkWW_sig_m0_CMS_jj_WZNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_WZNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_WZNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_WZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_WZNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_WZNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZNP, jj_BulkWW_sig_gsigma_CMS_jj_WZNP);
jjCBSigBulkWW_CMS_jj_WZNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_WZNP, jj_BulkWW_sig_sigma_CMS_jj_WZNP, jj_BulkWW_sig_alpha_CMS_jj_WZNP, jj_BulkWW_sig_n_CMS_jj_WZNP);
BulkWW_jj_CMS_jj_WZNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_WZNP, jjCBSigBulkWW_CMS_jj_WZNP, jj_BulkWW_sig_frac_CMS_jj_WZNP);

jj_BulkWW_sig_m0_CMS_jj_ZZNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_ZZNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_ZZNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_ZZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_ZZNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_ZZNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZNP, jj_BulkWW_sig_gsigma_CMS_jj_ZZNP);
jjCBSigBulkWW_CMS_jj_ZZNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_ZZNP, jj_BulkWW_sig_sigma_CMS_jj_ZZNP, jj_BulkWW_sig_alpha_CMS_jj_ZZNP, jj_BulkWW_sig_n_CMS_jj_ZZNP);
BulkWW_jj_CMS_jj_ZZNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_ZZNP, jjCBSigBulkWW_CMS_jj_ZZNP, jj_BulkWW_sig_frac_CMS_jj_ZZNP);



jj_BulkWW_sig_m0_CMS_jj_qVHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVHP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkWW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_sigma_CMS_jj_qVHP, jj_BulkWW_sig_alpha_CMS_jj_qVHP, jj_BulkWW_sig_n_CMS_jj_qVHP);
BulkWW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVHP, jjCBSigBulkWW_CMS_jj_qVHP, jj_BulkWW_sig_frac_CMS_jj_qVHP);

jj_BulkWW_sig_m0_CMS_jj_qWHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qWHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qWHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qWHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qWHP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qWHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWHP, jj_BulkWW_sig_gsigma_CMS_jj_qWHP);
jjCBSigBulkWW_CMS_jj_qWHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWHP, jj_BulkWW_sig_sigma_CMS_jj_qWHP, jj_BulkWW_sig_alpha_CMS_jj_qWHP, jj_BulkWW_sig_n_CMS_jj_qWHP);
BulkWW_jj_CMS_jj_qWHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qWHP, jjCBSigBulkWW_CMS_jj_qWHP, jj_BulkWW_sig_frac_CMS_jj_qWHP);

jj_BulkWW_sig_m0_CMS_jj_qZHP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qZHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qZHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qZHP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qZHP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qZHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZHP, jj_BulkWW_sig_gsigma_CMS_jj_qZHP);
jjCBSigBulkWW_CMS_jj_qZHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZHP, jj_BulkWW_sig_sigma_CMS_jj_qZHP, jj_BulkWW_sig_alpha_CMS_jj_qZHP, jj_BulkWW_sig_n_CMS_jj_qZHP);
BulkWW_jj_CMS_jj_qZHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qZHP, jjCBSigBulkWW_CMS_jj_qZHP, jj_BulkWW_sig_frac_CMS_jj_qZHP);



jj_BulkWW_sig_m0_CMS_jj_qVLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkWW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_sigma_CMS_jj_qVLP, jj_BulkWW_sig_alpha_CMS_jj_qVLP, jj_BulkWW_sig_n_CMS_jj_qVLP);
BulkWW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVLP, jjCBSigBulkWW_CMS_jj_qVLP, jj_BulkWW_sig_frac_CMS_jj_qVLP);

jj_BulkWW_sig_m0_CMS_jj_qWLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qWLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qWLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qWLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qWLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qWLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWLP, jj_BulkWW_sig_gsigma_CMS_jj_qWLP);
jjCBSigBulkWW_CMS_jj_qWLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWLP, jj_BulkWW_sig_sigma_CMS_jj_qWLP, jj_BulkWW_sig_alpha_CMS_jj_qWLP, jj_BulkWW_sig_n_CMS_jj_qWLP);
BulkWW_jj_CMS_jj_qWLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qWLP, jjCBSigBulkWW_CMS_jj_qWLP, jj_BulkWW_sig_frac_CMS_jj_qWLP);

jj_BulkWW_sig_m0_CMS_jj_qZLP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qZLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qZLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qZLP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qZLP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qZLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZLP, jj_BulkWW_sig_gsigma_CMS_jj_qZLP);
jjCBSigBulkWW_CMS_jj_qZLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZLP, jj_BulkWW_sig_sigma_CMS_jj_qZLP, jj_BulkWW_sig_alpha_CMS_jj_qZLP, jj_BulkWW_sig_n_CMS_jj_qZLP);
BulkWW_jj_CMS_jj_qZLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qZLP, jjCBSigBulkWW_CMS_jj_qZLP, jj_BulkWW_sig_frac_CMS_jj_qZLP);


jj_BulkWW_sig_m0_CMS_jj_qVNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkWW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_sigma_CMS_jj_qVNP, jj_BulkWW_sig_alpha_CMS_jj_qVNP, jj_BulkWW_sig_n_CMS_jj_qVNP);
BulkWW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVNP, jjCBSigBulkWW_CMS_jj_qVNP, jj_BulkWW_sig_frac_CMS_jj_qVNP);


jj_BulkWW_sig_m0_CMS_jj_qWNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qWNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qWNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qWNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qWNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qWNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWNP, jj_BulkWW_sig_gsigma_CMS_jj_qWNP);
jjCBSigBulkWW_CMS_jj_qWNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qWNP, jj_BulkWW_sig_sigma_CMS_jj_qWNP, jj_BulkWW_sig_alpha_CMS_jj_qWNP, jj_BulkWW_sig_n_CMS_jj_qWNP);
BulkWW_jj_CMS_jj_qWNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qWNP, jjCBSigBulkWW_CMS_jj_qWNP, jj_BulkWW_sig_frac_CMS_jj_qWNP);

jj_BulkWW_sig_m0_CMS_jj_qZNP[2000.0, 1000.0, 4000.0];
jj_BulkWW_sig_sigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qZNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qZNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qZNP[100, 0.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qZNP[0.5, 0.0, 1.0];

jjGaussSigBulkWW_CMS_jj_qZNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZNP, jj_BulkWW_sig_gsigma_CMS_jj_qZNP);
jjCBSigBulkWW_CMS_jj_qZNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qZNP, jj_BulkWW_sig_sigma_CMS_jj_qZNP, jj_BulkWW_sig_alpha_CMS_jj_qZNP, jj_BulkWW_sig_n_CMS_jj_qZNP);
BulkWW_jj_CMS_jj_qZNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qZNP, jjCBSigBulkWW_CMS_jj_qZNP, jj_BulkWW_sig_frac_CMS_jj_qZNP);


#For 2-parameter fit
bkg_fit_slope_CMS_jj_WWHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WWHP[0., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_WWHP[4.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WWHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_WZHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WZHP[0., -100., 500.0];
bkg_fit_slope2_CMS_jj_WZHP[4.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WZHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_WWLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WWLP[0., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_WWLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WWLP[0.,-50.0, 50.0];


##For 3 parameter fit
bkg_fit_slope[1.0,0, 1];
bkg_fit_slope1[7,0.0, 100.0];
bkg_fit_slope2[5,0.0, 100.0];
bkg_fit_slope3[0.,-100.0, 100.0];

bkg_fit_slope_CMS_jj_VVHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVHP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVHP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_VVHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_ZZHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_ZZHP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_ZZHP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_ZZHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_VVLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_VVLP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_WZLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WZLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_WZLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WZLP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_ZZLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_ZZLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_ZZLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_ZZLP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_VVNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_VVNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_WWNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WWNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_WWNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WWNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_WZNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_WZNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_WZNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_WZNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_ZZNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_ZZNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_ZZNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_ZZNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qVHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVHP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVHP[3.,-100.0, 100.0];];
bkg_fit_slope3_CMS_jj_qVHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qWHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qWHP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qWHP[3.,-100.0, 100.0];];
bkg_fit_slope3_CMS_jj_qWHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qZHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qZHP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qZHP[3.,-100.0, 100.0];];
bkg_fit_slope3_CMS_jj_qZHP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qVLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qVLP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qWLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qWLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qWLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qWLP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qZLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qZLP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qZLP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qZLP[0.,-50.0, 50.0];


bkg_fit_slope_CMS_jj_qVNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qVNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qWNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qWNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qWNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qWNP[0.,-50.0, 50.0];

bkg_fit_slope_CMS_jj_qZNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qZNP[20., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qZNP[3.,-100.0, 100.0];
bkg_fit_slope3_CMS_jj_qZNP[0.,-50.0, 50.0];

#parameters for alternative 3-parameter function
#bkg_fit_slope_CMS_jj_VVHP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVHP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVHP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVHP[0.,-50.0, 50.0];

#bkg_fit_slope_CMS_jj_VVLP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVLP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVLP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVLP[0.,-50.0, 50.0];

#bkg_fit_slope_CMS_jj_VVNP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVNP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVNP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVNP[0.,-50.0, 50.0];

#bkg_fit_slope_CMS_jj_qVHP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVHP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVHP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVHP[0.,-50.0, 50.0];

#bkg_fit_slope_CMS_jj_qVLP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVLP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVLP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVLP[0.,-50.0, 50.0];

#bkg_fit_slope_CMS_jj_qVNP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVNP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVNP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVNP[0.,-50.0, 50.0];

wei[1,0,10];

sqrtS[13000., 13000., 13000.]
