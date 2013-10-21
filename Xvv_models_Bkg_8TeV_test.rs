mgg[890,2500];

jj_RS1ZZ_sig_m0[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha[0.8, 0.5, 3]; 
jj_RS1ZZ_sig_n[13.0, 0.5, 10]; 
jj_RS1ZZ_sig_gsigma[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ = Gaussian(mgg, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_gsigma);
jjCBSigRS1ZZ    = CBShape(mgg, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_sigma, jj_RS1ZZ_sig_alpha, jj_RS1ZZ_sig_n);
sig_RS1ZZ      = AddPdf(jjGaussSigRS1ZZ, jjCBSigRS1ZZ, jj_RS1ZZ_sig_frac);

jj_RS1ZZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigRS1ZZ_CMS_jj_VVHP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1ZZ_CMS_jj_VVHP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_sigma_CMS_jj_VVHP, jj_RS1ZZ_sig_alpha_CMS_jj_VVHP, jj_RS1ZZ_sig_n_CMS_jj_VVHP);
sig_RS1ZZ_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVHP, jjCBSigRS1ZZ_CMS_jj_VVHP, jj_RS1ZZ_sig_frac_CMS_jj_VVHP);

jj_RS1ZZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVLP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1ZZ_CMS_jj_VVLP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_sigma_CMS_jj_VVLP, jj_RS1ZZ_sig_alpha_CMS_jj_VVLP, jj_RS1ZZ_sig_n_CMS_jj_VVLP);
sig_RS1ZZ_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVLP, jjCBSigRS1ZZ_CMS_jj_VVLP, jj_RS1ZZ_sig_frac_CMS_jj_VVLP);

jj_RS1ZZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVNP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1ZZ_CMS_jj_VVNP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_sigma_CMS_jj_VVNP, jj_RS1ZZ_sig_alpha_CMS_jj_VVNP, jj_RS1ZZ_sig_n_CMS_jj_VVNP);
sig_RS1ZZ_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVNP, jjCBSigRS1ZZ_CMS_jj_VVNP, jj_RS1ZZ_sig_frac_CMS_jj_VVNP);

jj_RS1ZZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVHP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1ZZ_CMS_jj_qVHP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_sigma_CMS_jj_qVHP, jj_RS1ZZ_sig_alpha_CMS_jj_qVHP, jj_RS1ZZ_sig_n_CMS_jj_qVHP);
sig_RS1ZZ_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVHP, jjCBSigRS1ZZ_CMS_jj_qVHP, jj_RS1ZZ_sig_frac_CMS_jj_qVHP);

jj_RS1ZZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVLP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1ZZ_CMS_jj_qVLP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_sigma_CMS_jj_qVLP, jj_RS1ZZ_sig_alpha_CMS_jj_qVLP, jj_RS1ZZ_sig_n_CMS_jj_qVLP);
sig_RS1ZZ_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVLP, jjCBSigRS1ZZ_CMS_jj_qVLP, jj_RS1ZZ_sig_frac_CMS_jj_qVLP);


jj_RS1ZZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 3100.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVNP = Gaussian(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1ZZ_CMS_jj_qVNP    = CBShape(mgg, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_sigma_CMS_jj_qVNP, jj_RS1ZZ_sig_alpha_CMS_jj_qVNP, jj_RS1ZZ_sig_n_CMS_jj_qVNP);
sig_RS1ZZ_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVNP, jjCBSigRS1ZZ_CMS_jj_qVNP, jj_RS1ZZ_sig_frac_CMS_jj_qVNP);






jj_RS1WW_sig_m0[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha[0.8, 0.5, 3]; 
jj_RS1WW_sig_n[13.0, 0.5, 10]; 
jj_RS1WW_sig_gsigma[100, 50.0, 1000.0];
jj_RS1WW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigRS1WW = Gaussian(mgg, jj_RS1WW_sig_m0, jj_RS1WW_sig_gsigma);
jjCBSigRS1WW    = CBShape(mgg, jj_RS1WW_sig_m0, jj_RS1WW_sig_sigma, jj_RS1WW_sig_alpha, jj_RS1WW_sig_n);
sig_RS1WW      = AddPdf(jjGaussSigRS1WW, jjCBSigRS1WW, jj_RS1WW_sig_frac);

jj_RS1WW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigRS1WW_CMS_jj_VVHP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1WW_CMS_jj_VVHP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_sigma_CMS_jj_VVHP, jj_RS1WW_sig_alpha_CMS_jj_VVHP, jj_RS1WW_sig_n_CMS_jj_VVHP);
sig_RS1WW_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVHP, jjCBSigRS1WW_CMS_jj_VVHP, jj_RS1WW_sig_frac_CMS_jj_VVHP);

jj_RS1WW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_VVLP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1WW_CMS_jj_VVLP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_sigma_CMS_jj_VVLP, jj_RS1WW_sig_alpha_CMS_jj_VVLP, jj_RS1WW_sig_n_CMS_jj_VVLP);
sig_RS1WW_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVLP, jjCBSigRS1WW_CMS_jj_VVLP, jj_RS1WW_sig_frac_CMS_jj_VVLP);

jj_RS1WW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_VVNP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1WW_CMS_jj_VVNP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_sigma_CMS_jj_VVNP, jj_RS1WW_sig_alpha_CMS_jj_VVNP, jj_RS1WW_sig_n_CMS_jj_VVNP);
sig_RS1WW_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVNP, jjCBSigRS1WW_CMS_jj_VVNP, jj_RS1WW_sig_frac_CMS_jj_VVNP);

jj_RS1WW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVHP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1WW_CMS_jj_qVHP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_sigma_CMS_jj_qVHP, jj_RS1WW_sig_alpha_CMS_jj_qVHP, jj_RS1WW_sig_n_CMS_jj_qVHP);
sig_RS1WW_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVHP, jjCBSigRS1WW_CMS_jj_qVHP, jj_RS1WW_sig_frac_CMS_jj_qVHP);

jj_RS1WW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVLP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1WW_CMS_jj_qVLP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_sigma_CMS_jj_qVLP, jj_RS1WW_sig_alpha_CMS_jj_qVLP, jj_RS1WW_sig_n_CMS_jj_qVLP);
sig_RS1WW_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVLP, jjCBSigRS1WW_CMS_jj_qVLP, jj_RS1WW_sig_frac_CMS_jj_qVLP);


jj_RS1WW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 3100.0];
jj_RS1WW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVNP = Gaussian(mgg, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1WW_CMS_jj_qVNP    = CBShape(mgg, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_sigma_CMS_jj_qVNP, jj_RS1WW_sig_alpha_CMS_jj_qVNP, jj_RS1WW_sig_n_CMS_jj_qVNP);
sig_RS1WW_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVNP, jjCBSigRS1WW_CMS_jj_qVNP, jj_RS1WW_sig_frac_CMS_jj_qVNP);






jj_WZ_sig_m0[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma[100, 50.0, 1000.0];
jj_WZ_sig_alpha[0.8, 0.5, 3]; 
jj_WZ_sig_n[13.0, 0.5, 10]; 
jj_WZ_sig_gsigma[100, 50.0, 1000.0];
jj_WZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigWZ = Gaussian(mgg, jj_WZ_sig_m0, jj_WZ_sig_gsigma);
jjCBSigWZ    = CBShape(mgg, jj_WZ_sig_m0, jj_WZ_sig_sigma, jj_WZ_sig_alpha, jj_WZ_sig_n);
sig_WZ      = AddPdf(jjGaussSigWZ, jjCBSigWZ, jj_WZ_sig_frac);

jj_WZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigWZ_CMS_jj_VVHP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigWZ_CMS_jj_VVHP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_sigma_CMS_jj_VVHP, jj_WZ_sig_alpha_CMS_jj_VVHP, jj_WZ_sig_n_CMS_jj_VVHP);
sig_WZ_CMS_jj_VVHP      = AddPdf(jjGaussSigWZ_CMS_jj_VVHP, jjCBSigWZ_CMS_jj_VVHP, jj_WZ_sig_frac_CMS_jj_VVHP);

jj_WZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_VVLP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigWZ_CMS_jj_VVLP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_sigma_CMS_jj_VVLP, jj_WZ_sig_alpha_CMS_jj_VVLP, jj_WZ_sig_n_CMS_jj_VVLP);
sig_WZ_CMS_jj_VVLP      = AddPdf(jjGaussSigWZ_CMS_jj_VVLP, jjCBSigWZ_CMS_jj_VVLP, jj_WZ_sig_frac_CMS_jj_VVLP);

jj_WZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_VVNP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigWZ_CMS_jj_VVNP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_sigma_CMS_jj_VVNP, jj_WZ_sig_alpha_CMS_jj_VVNP, jj_WZ_sig_n_CMS_jj_VVNP);
sig_WZ_CMS_jj_VVNP      = AddPdf(jjGaussSigWZ_CMS_jj_VVNP, jjCBSigWZ_CMS_jj_VVNP, jj_WZ_sig_frac_CMS_jj_VVNP);

jj_WZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVHP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigWZ_CMS_jj_qVHP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_sigma_CMS_jj_qVHP, jj_WZ_sig_alpha_CMS_jj_qVHP, jj_WZ_sig_n_CMS_jj_qVHP);
sig_WZ_CMS_jj_qVHP      = AddPdf(jjGaussSigWZ_CMS_jj_qVHP, jjCBSigWZ_CMS_jj_qVHP, jj_WZ_sig_frac_CMS_jj_qVHP);

jj_WZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVLP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigWZ_CMS_jj_qVLP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_sigma_CMS_jj_qVLP, jj_WZ_sig_alpha_CMS_jj_qVLP, jj_WZ_sig_n_CMS_jj_qVLP);
sig_WZ_CMS_jj_qVLP      = AddPdf(jjGaussSigWZ_CMS_jj_qVLP, jjCBSigWZ_CMS_jj_qVLP, jj_WZ_sig_frac_CMS_jj_qVLP);


jj_WZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 3100.0];
jj_WZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVNP = Gaussian(mgg, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigWZ_CMS_jj_qVNP    = CBShape(mgg, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_sigma_CMS_jj_qVNP, jj_WZ_sig_alpha_CMS_jj_qVNP, jj_WZ_sig_n_CMS_jj_qVNP);
sig_WZ_CMS_jj_qVNP      = AddPdf(jjGaussSigWZ_CMS_jj_qVNP, jjCBSigWZ_CMS_jj_qVNP, jj_WZ_sig_frac_CMS_jj_qVNP);






jj_qW_sig_m0[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma[100, 50.0, 1000.0];
jj_qW_sig_alpha[0.8, 0.5, 3]; 
jj_qW_sig_n[13.0, 0.5, 10]; 
jj_qW_sig_gsigma[100, 50.0, 1000.0];
jj_qW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigqW = Gaussian(mgg, jj_qW_sig_m0, jj_qW_sig_gsigma);
jjCBSigqW    = CBShape(mgg, jj_qW_sig_m0, jj_qW_sig_sigma, jj_qW_sig_alpha, jj_qW_sig_n);
sig_qW      = AddPdf(jjGaussSigqW, jjCBSigqW, jj_qW_sig_frac);

jj_qW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigqW_CMS_jj_VVHP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_gsigma_CMS_jj_VVHP);
jjCBSigqW_CMS_jj_VVHP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_sigma_CMS_jj_VVHP, jj_qW_sig_alpha_CMS_jj_VVHP, jj_qW_sig_n_CMS_jj_VVHP);
sig_qW_CMS_jj_VVHP      = AddPdf(jjGaussSigqW_CMS_jj_VVHP, jjCBSigqW_CMS_jj_VVHP, jj_qW_sig_frac_CMS_jj_VVHP);

jj_qW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_VVLP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_gsigma_CMS_jj_VVLP);
jjCBSigqW_CMS_jj_VVLP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_sigma_CMS_jj_VVLP, jj_qW_sig_alpha_CMS_jj_VVLP, jj_qW_sig_n_CMS_jj_VVLP);
sig_qW_CMS_jj_VVLP      = AddPdf(jjGaussSigqW_CMS_jj_VVLP, jjCBSigqW_CMS_jj_VVLP, jj_qW_sig_frac_CMS_jj_VVLP);

jj_qW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_VVNP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_gsigma_CMS_jj_VVNP);
jjCBSigqW_CMS_jj_VVNP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_sigma_CMS_jj_VVNP, jj_qW_sig_alpha_CMS_jj_VVNP, jj_qW_sig_n_CMS_jj_VVNP);
sig_qW_CMS_jj_VVNP      = AddPdf(jjGaussSigqW_CMS_jj_VVNP, jjCBSigqW_CMS_jj_VVNP, jj_qW_sig_frac_CMS_jj_VVNP);

jj_qW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVHP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_gsigma_CMS_jj_qVHP);
jjCBSigqW_CMS_jj_qVHP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_sigma_CMS_jj_qVHP, jj_qW_sig_alpha_CMS_jj_qVHP, jj_qW_sig_n_CMS_jj_qVHP);
sig_qW_CMS_jj_qVHP      = AddPdf(jjGaussSigqW_CMS_jj_qVHP, jjCBSigqW_CMS_jj_qVHP, jj_qW_sig_frac_CMS_jj_qVHP);

jj_qW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVLP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_gsigma_CMS_jj_qVLP);
jjCBSigqW_CMS_jj_qVLP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_sigma_CMS_jj_qVLP, jj_qW_sig_alpha_CMS_jj_qVLP, jj_qW_sig_n_CMS_jj_qVLP);
sig_qW_CMS_jj_qVLP      = AddPdf(jjGaussSigqW_CMS_jj_qVLP, jjCBSigqW_CMS_jj_qVLP, jj_qW_sig_frac_CMS_jj_qVLP);


jj_qW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVNP = Gaussian(mgg, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_gsigma_CMS_jj_qVNP);
jjCBSigqW_CMS_jj_qVNP    = CBShape(mgg, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_sigma_CMS_jj_qVNP, jj_qW_sig_alpha_CMS_jj_qVNP, jj_qW_sig_n_CMS_jj_qVNP);
sig_qW_CMS_jj_qVNP      = AddPdf(jjGaussSigqW_CMS_jj_qVNP, jjCBSigqW_CMS_jj_qVNP, jj_qW_sig_frac_CMS_jj_qVNP);






jj_qZ_sig_m0[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma[100, 50.0, 1000.0];
jj_qZ_sig_alpha[0.8, 0.5, 3]; 
jj_qZ_sig_n[13.0, 0.5, 10]; 
jj_qZ_sig_gsigma[100, 50.0, 1000.0];
jj_qZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigqZ = Gaussian(mgg, jj_qZ_sig_m0, jj_qZ_sig_gsigma);
jjCBSigqZ    = CBShape(mgg, jj_qZ_sig_m0, jj_qZ_sig_sigma, jj_qZ_sig_alpha, jj_qZ_sig_n);
sig_qZ      = AddPdf(jjGaussSigqZ, jjCBSigqZ, jj_qZ_sig_frac);

jj_qZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigqZ_CMS_jj_VVHP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigqZ_CMS_jj_VVHP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_sigma_CMS_jj_VVHP, jj_qZ_sig_alpha_CMS_jj_VVHP, jj_qZ_sig_n_CMS_jj_VVHP);
sig_qZ_CMS_jj_VVHP      = AddPdf(jjGaussSigqZ_CMS_jj_VVHP, jjCBSigqZ_CMS_jj_VVHP, jj_qZ_sig_frac_CMS_jj_VVHP);

jj_qZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_VVLP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigqZ_CMS_jj_VVLP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_sigma_CMS_jj_VVLP, jj_qZ_sig_alpha_CMS_jj_VVLP, jj_qZ_sig_n_CMS_jj_VVLP);
sig_qZ_CMS_jj_VVLP      = AddPdf(jjGaussSigqZ_CMS_jj_VVLP, jjCBSigqZ_CMS_jj_VVLP, jj_qZ_sig_frac_CMS_jj_VVLP);

jj_qZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_VVNP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigqZ_CMS_jj_VVNP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_sigma_CMS_jj_VVNP, jj_qZ_sig_alpha_CMS_jj_VVNP, jj_qZ_sig_n_CMS_jj_VVNP);
sig_qZ_CMS_jj_VVNP      = AddPdf(jjGaussSigqZ_CMS_jj_VVNP, jjCBSigqZ_CMS_jj_VVNP, jj_qZ_sig_frac_CMS_jj_VVNP);

jj_qZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVHP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigqZ_CMS_jj_qVHP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_sigma_CMS_jj_qVHP, jj_qZ_sig_alpha_CMS_jj_qVHP, jj_qZ_sig_n_CMS_jj_qVHP);
sig_qZ_CMS_jj_qVHP      = AddPdf(jjGaussSigqZ_CMS_jj_qVHP, jjCBSigqZ_CMS_jj_qVHP, jj_qZ_sig_frac_CMS_jj_qVHP);

jj_qZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVLP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigqZ_CMS_jj_qVLP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_sigma_CMS_jj_qVLP, jj_qZ_sig_alpha_CMS_jj_qVLP, jj_qZ_sig_n_CMS_jj_qVLP);
sig_qZ_CMS_jj_qVLP      = AddPdf(jjGaussSigqZ_CMS_jj_qVLP, jjCBSigqZ_CMS_jj_qVLP, jj_qZ_sig_frac_CMS_jj_qVLP);


jj_qZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVNP = Gaussian(mgg, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigqZ_CMS_jj_qVNP    = CBShape(mgg, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_sigma_CMS_jj_qVNP, jj_qZ_sig_alpha_CMS_jj_qVNP, jj_qZ_sig_n_CMS_jj_qVNP);
sig_qZ_CMS_jj_qVNP      = AddPdf(jjGaussSigqZ_CMS_jj_qVNP, jjCBSigqZ_CMS_jj_qVNP, jj_qZ_sig_frac_CMS_jj_qVNP);





jj_BulkZZ_sig_m0[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha[0.8, 0.5, 3]; 
jj_BulkZZ_sig_n[13.0, 0.5, 10]; 
jj_BulkZZ_sig_gsigma[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigBulkZZ = Gaussian(mgg, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_gsigma);
jjCBSigBulkZZ    = CBShape(mgg, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_sigma, jj_BulkZZ_sig_alpha, jj_BulkZZ_sig_n);
sig_BulkZZ      = AddPdf(jjGaussSigBulkZZ, jjCBSigBulkZZ, jj_BulkZZ_sig_frac);

jj_BulkZZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigBulkZZ_CMS_jj_VVHP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkZZ_CMS_jj_VVHP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_sigma_CMS_jj_VVHP, jj_BulkZZ_sig_alpha_CMS_jj_VVHP, jj_BulkZZ_sig_n_CMS_jj_VVHP);
sig_BulkZZ_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVHP, jjCBSigBulkZZ_CMS_jj_VVHP, jj_BulkZZ_sig_frac_CMS_jj_VVHP);

jj_BulkZZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVLP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkZZ_CMS_jj_VVLP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_sigma_CMS_jj_VVLP, jj_BulkZZ_sig_alpha_CMS_jj_VVLP, jj_BulkZZ_sig_n_CMS_jj_VVLP);
sig_BulkZZ_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVLP, jjCBSigBulkZZ_CMS_jj_VVLP, jj_BulkZZ_sig_frac_CMS_jj_VVLP);

jj_BulkZZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVNP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkZZ_CMS_jj_VVNP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_sigma_CMS_jj_VVNP, jj_BulkZZ_sig_alpha_CMS_jj_VVNP, jj_BulkZZ_sig_n_CMS_jj_VVNP);
sig_BulkZZ_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVNP, jjCBSigBulkZZ_CMS_jj_VVNP, jj_BulkZZ_sig_frac_CMS_jj_VVNP);

jj_BulkZZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVHP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkZZ_CMS_jj_qVHP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_sigma_CMS_jj_qVHP, jj_BulkZZ_sig_alpha_CMS_jj_qVHP, jj_BulkZZ_sig_n_CMS_jj_qVHP);
sig_BulkZZ_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVHP, jjCBSigBulkZZ_CMS_jj_qVHP, jj_BulkZZ_sig_frac_CMS_jj_qVHP);

jj_BulkZZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVLP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkZZ_CMS_jj_qVLP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_sigma_CMS_jj_qVLP, jj_BulkZZ_sig_alpha_CMS_jj_qVLP, jj_BulkZZ_sig_n_CMS_jj_qVLP);
sig_BulkZZ_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVLP, jjCBSigBulkZZ_CMS_jj_qVLP, jj_BulkZZ_sig_frac_CMS_jj_qVLP);


jj_BulkZZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 3100.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVNP = Gaussian(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkZZ_CMS_jj_qVNP    = CBShape(mgg, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_sigma_CMS_jj_qVNP, jj_BulkZZ_sig_alpha_CMS_jj_qVNP, jj_BulkZZ_sig_n_CMS_jj_qVNP);
sig_BulkZZ_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVNP, jjCBSigBulkZZ_CMS_jj_qVNP, jj_BulkZZ_sig_frac_CMS_jj_qVNP);






jj_BulkWW_sig_m0[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha[0.8, 0.5, 3]; 
jj_BulkWW_sig_n[13.0, 0.5, 10]; 
jj_BulkWW_sig_gsigma[100, 50.0, 1000.0];
jj_BulkWW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigBulkWW = Gaussian(mgg, jj_BulkWW_sig_m0, jj_BulkWW_sig_gsigma);
jjCBSigBulkWW    = CBShape(mgg, jj_BulkWW_sig_m0, jj_BulkWW_sig_sigma, jj_BulkWW_sig_alpha, jj_BulkWW_sig_n);
sig_BulkWW      = AddPdf(jjGaussSigBulkWW, jjCBSigBulkWW, jj_BulkWW_sig_frac);

jj_BulkWW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigBulkWW_CMS_jj_VVHP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkWW_CMS_jj_VVHP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_sigma_CMS_jj_VVHP, jj_BulkWW_sig_alpha_CMS_jj_VVHP, jj_BulkWW_sig_n_CMS_jj_VVHP);
sig_BulkWW_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVHP, jjCBSigBulkWW_CMS_jj_VVHP, jj_BulkWW_sig_frac_CMS_jj_VVHP);

jj_BulkWW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_VVLP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkWW_CMS_jj_VVLP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_sigma_CMS_jj_VVLP, jj_BulkWW_sig_alpha_CMS_jj_VVLP, jj_BulkWW_sig_n_CMS_jj_VVLP);
sig_BulkWW_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVLP, jjCBSigBulkWW_CMS_jj_VVLP, jj_BulkWW_sig_frac_CMS_jj_VVLP);

jj_BulkWW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_VVNP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkWW_CMS_jj_VVNP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_sigma_CMS_jj_VVNP, jj_BulkWW_sig_alpha_CMS_jj_VVNP, jj_BulkWW_sig_n_CMS_jj_VVNP);
sig_BulkWW_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVNP, jjCBSigBulkWW_CMS_jj_VVNP, jj_BulkWW_sig_frac_CMS_jj_VVNP);

jj_BulkWW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVHP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkWW_CMS_jj_qVHP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_sigma_CMS_jj_qVHP, jj_BulkWW_sig_alpha_CMS_jj_qVHP, jj_BulkWW_sig_n_CMS_jj_qVHP);
sig_BulkWW_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVHP, jjCBSigBulkWW_CMS_jj_qVHP, jj_BulkWW_sig_frac_CMS_jj_qVHP);

jj_BulkWW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVLP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkWW_CMS_jj_qVLP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_sigma_CMS_jj_qVLP, jj_BulkWW_sig_alpha_CMS_jj_qVLP, jj_BulkWW_sig_n_CMS_jj_qVLP);
sig_BulkWW_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVLP, jjCBSigBulkWW_CMS_jj_qVLP, jj_BulkWW_sig_frac_CMS_jj_qVLP);


jj_BulkWW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 3100.0];
jj_BulkWW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVNP = Gaussian(mgg, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkWW_CMS_jj_qVNP    = CBShape(mgg, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_sigma_CMS_jj_qVNP, jj_BulkWW_sig_alpha_CMS_jj_qVNP, jj_BulkWW_sig_n_CMS_jj_qVNP);
sig_BulkWW_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVNP, jjCBSigBulkWW_CMS_jj_qVNP, jj_BulkWW_sig_frac_CMS_jj_qVNP);






bkg_fit_slope[1.0,0, 1];
bkg_fit_slope1[7,0.0, 100.0];
bkg_fit_slope2[5,0.0, 100.0];
bkg_fit_slope3[0.,0.0, 0.0];


bkg_fit_slope_CMS_jj_VVHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVHP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVHP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_VVHP[0.,-10.0, 10.0];

bkg_fit_slope_CMS_jj_VVLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVLP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVLP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_VVLP[0.,-10.0, 10.0];

bkg_fit_slope_CMS_jj_VVNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_VVNP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_VVNP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_VVNP[0.,-10.0, 10.0];

bkg_fit_slope_CMS_jj_qVHP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVHP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVHP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_qVHP[0.,-10.0, 10.0];

bkg_fit_slope_CMS_jj_qVLP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVLP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVLP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_qVLP[0.,-10.0, 10.0];

bkg_fit_slope_CMS_jj_qVNP[1000.0,0, 10000000];
bkg_fit_slope1_CMS_jj_qVNP[10., -100.0, 500.0];
bkg_fit_slope2_CMS_jj_qVNP[5.,0.0, 100.0];
bkg_fit_slope3_CMS_jj_qVNP[0.,-10.0, 10.0];

#parameters for alternative 3-parameter function
#bkg_fit_slope_CMS_jj_VVHP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVHP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVHP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVHP[0.,-10.0, 10.0];

#bkg_fit_slope_CMS_jj_VVLP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVLP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVLP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVLP[0.,-10.0, 10.0];

#bkg_fit_slope_CMS_jj_VVNP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_VVNP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_VVNP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_VVNP[0.,-10.0, 10.0];

#bkg_fit_slope_CMS_jj_qVHP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVHP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVHP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVHP[0.,-10.0, 10.0];

#bkg_fit_slope_CMS_jj_qVLP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVLP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVLP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVLP[0.,-10.0, 10.0];

#bkg_fit_slope_CMS_jj_qVNP[10000.0,0, 100000000];
#bkg_fit_slope1_CMS_jj_qVNP[3.0, 1.0, 10.0];
#bkg_fit_slope2_CMS_jj_qVNP[0.3, 0.0, 10.0];
#bkg_fit_slope3_CMS_jj_qVNP[0.,-10.0, 10.0];

wei[1,0,10];

sqrtS[8000., 8000., 8000.]
