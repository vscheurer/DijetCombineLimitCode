mgg13TeV[1000,5000];

jj_RS1ZZ_sig_m0[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha[0.8, 0.5, 3]; 
jj_RS1ZZ_sig_n[13.0, 0.5, 10]; 
jj_RS1ZZ_sig_gsigma[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_gsigma);
jjCBSigRS1ZZ    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0, jj_RS1ZZ_sig_sigma, jj_RS1ZZ_sig_alpha, jj_RS1ZZ_sig_n);
RS1ZZ_jj      = AddPdf(jjGaussSigRS1ZZ, jjCBSigRS1ZZ, jj_RS1ZZ_sig_frac);

jj_RS1ZZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigRS1ZZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1ZZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVHP, jj_RS1ZZ_sig_sigma_CMS_jj_VVHP, jj_RS1ZZ_sig_alpha_CMS_jj_VVHP, jj_RS1ZZ_sig_n_CMS_jj_VVHP);
RS1ZZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVHP, jjCBSigRS1ZZ_CMS_jj_VVHP, jj_RS1ZZ_sig_frac_CMS_jj_VVHP);

jj_RS1ZZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1ZZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVLP, jj_RS1ZZ_sig_sigma_CMS_jj_VVLP, jj_RS1ZZ_sig_alpha_CMS_jj_VVLP, jj_RS1ZZ_sig_n_CMS_jj_VVLP);
RS1ZZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVLP, jjCBSigRS1ZZ_CMS_jj_VVLP, jj_RS1ZZ_sig_frac_CMS_jj_VVLP);

jj_RS1ZZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1ZZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_VVNP, jj_RS1ZZ_sig_sigma_CMS_jj_VVNP, jj_RS1ZZ_sig_alpha_CMS_jj_VVNP, jj_RS1ZZ_sig_n_CMS_jj_VVNP);
RS1ZZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_VVNP, jjCBSigRS1ZZ_CMS_jj_VVNP, jj_RS1ZZ_sig_frac_CMS_jj_VVNP);

jj_RS1ZZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1ZZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVHP, jj_RS1ZZ_sig_sigma_CMS_jj_qVHP, jj_RS1ZZ_sig_alpha_CMS_jj_qVHP, jj_RS1ZZ_sig_n_CMS_jj_qVHP);
RS1ZZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVHP, jjCBSigRS1ZZ_CMS_jj_qVHP, jj_RS1ZZ_sig_frac_CMS_jj_qVHP);

jj_RS1ZZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1ZZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVLP, jj_RS1ZZ_sig_sigma_CMS_jj_qVLP, jj_RS1ZZ_sig_alpha_CMS_jj_qVLP, jj_RS1ZZ_sig_n_CMS_jj_qVLP);
RS1ZZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVLP, jjCBSigRS1ZZ_CMS_jj_qVLP, jj_RS1ZZ_sig_frac_CMS_jj_qVLP);


jj_RS1ZZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 6500.0];
jj_RS1ZZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1ZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1ZZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigRS1ZZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1ZZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_RS1ZZ_sig_m0_CMS_jj_qVNP, jj_RS1ZZ_sig_sigma_CMS_jj_qVNP, jj_RS1ZZ_sig_alpha_CMS_jj_qVNP, jj_RS1ZZ_sig_n_CMS_jj_qVNP);
RS1ZZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1ZZ_CMS_jj_qVNP, jjCBSigRS1ZZ_CMS_jj_qVNP, jj_RS1ZZ_sig_frac_CMS_jj_qVNP);






jj_RS1WW_sig_m0[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha[0.8, 0.5, 3]; 
jj_RS1WW_sig_n[13.0, 0.5, 10]; 
jj_RS1WW_sig_gsigma[100, 50.0, 1000.0];
jj_RS1WW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigRS1WW = Gaussian(mgg13TeV, jj_RS1WW_sig_m0, jj_RS1WW_sig_gsigma);
jjCBSigRS1WW    = CBShape(mgg13TeV, jj_RS1WW_sig_m0, jj_RS1WW_sig_sigma, jj_RS1WW_sig_alpha, jj_RS1WW_sig_n);
RS1WW_jj      = AddPdf(jjGaussSigRS1WW, jjCBSigRS1WW, jj_RS1WW_sig_frac);

jj_RS1WW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigRS1WW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_gsigma_CMS_jj_VVHP);
jjCBSigRS1WW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVHP, jj_RS1WW_sig_sigma_CMS_jj_VVHP, jj_RS1WW_sig_alpha_CMS_jj_VVHP, jj_RS1WW_sig_n_CMS_jj_VVHP);
RS1WW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVHP, jjCBSigRS1WW_CMS_jj_VVHP, jj_RS1WW_sig_frac_CMS_jj_VVHP);

jj_RS1WW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_gsigma_CMS_jj_VVLP);
jjCBSigRS1WW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVLP, jj_RS1WW_sig_sigma_CMS_jj_VVLP, jj_RS1WW_sig_alpha_CMS_jj_VVLP, jj_RS1WW_sig_n_CMS_jj_VVLP);
RS1WW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVLP, jjCBSigRS1WW_CMS_jj_VVLP, jj_RS1WW_sig_frac_CMS_jj_VVLP);

jj_RS1WW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_gsigma_CMS_jj_VVNP);
jjCBSigRS1WW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_VVNP, jj_RS1WW_sig_sigma_CMS_jj_VVNP, jj_RS1WW_sig_alpha_CMS_jj_VVNP, jj_RS1WW_sig_n_CMS_jj_VVNP);
RS1WW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_VVNP, jjCBSigRS1WW_CMS_jj_VVNP, jj_RS1WW_sig_frac_CMS_jj_VVNP);

jj_RS1WW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_gsigma_CMS_jj_qVHP);
jjCBSigRS1WW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVHP, jj_RS1WW_sig_sigma_CMS_jj_qVHP, jj_RS1WW_sig_alpha_CMS_jj_qVHP, jj_RS1WW_sig_n_CMS_jj_qVHP);
RS1WW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVHP, jjCBSigRS1WW_CMS_jj_qVHP, jj_RS1WW_sig_frac_CMS_jj_qVHP);

jj_RS1WW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_gsigma_CMS_jj_qVLP);
jjCBSigRS1WW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVLP, jj_RS1WW_sig_sigma_CMS_jj_qVLP, jj_RS1WW_sig_alpha_CMS_jj_qVLP, jj_RS1WW_sig_n_CMS_jj_qVLP);
RS1WW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVLP, jjCBSigRS1WW_CMS_jj_qVLP, jj_RS1WW_sig_frac_CMS_jj_qVLP);


jj_RS1WW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 6500.0];
jj_RS1WW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_RS1WW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_RS1WW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_RS1WW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigRS1WW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_gsigma_CMS_jj_qVNP);
jjCBSigRS1WW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_RS1WW_sig_m0_CMS_jj_qVNP, jj_RS1WW_sig_sigma_CMS_jj_qVNP, jj_RS1WW_sig_alpha_CMS_jj_qVNP, jj_RS1WW_sig_n_CMS_jj_qVNP);
RS1WW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigRS1WW_CMS_jj_qVNP, jjCBSigRS1WW_CMS_jj_qVNP, jj_RS1WW_sig_frac_CMS_jj_qVNP);






jj_WZ_sig_m0[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma[100, 50.0, 1000.0];
jj_WZ_sig_alpha[0.8, 0.5, 3]; 
jj_WZ_sig_n[13.0, 0.5, 10]; 
jj_WZ_sig_gsigma[100, 50.0, 1000.0];
jj_WZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigWZ = Gaussian(mgg13TeV, jj_WZ_sig_m0, jj_WZ_sig_gsigma);
jjCBSigWZ    = CBShape(mgg13TeV, jj_WZ_sig_m0, jj_WZ_sig_sigma, jj_WZ_sig_alpha, jj_WZ_sig_n);
WZ_jj      = AddPdf(jjGaussSigWZ, jjCBSigWZ, jj_WZ_sig_frac);

jj_WZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigWZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigWZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVHP, jj_WZ_sig_sigma_CMS_jj_VVHP, jj_WZ_sig_alpha_CMS_jj_VVHP, jj_WZ_sig_n_CMS_jj_VVHP);
WZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigWZ_CMS_jj_VVHP, jjCBSigWZ_CMS_jj_VVHP, jj_WZ_sig_frac_CMS_jj_VVHP);

jj_WZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigWZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVLP, jj_WZ_sig_sigma_CMS_jj_VVLP, jj_WZ_sig_alpha_CMS_jj_VVLP, jj_WZ_sig_n_CMS_jj_VVLP);
WZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigWZ_CMS_jj_VVLP, jjCBSigWZ_CMS_jj_VVLP, jj_WZ_sig_frac_CMS_jj_VVLP);

jj_WZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigWZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_VVNP, jj_WZ_sig_sigma_CMS_jj_VVNP, jj_WZ_sig_alpha_CMS_jj_VVNP, jj_WZ_sig_n_CMS_jj_VVNP);
WZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigWZ_CMS_jj_VVNP, jjCBSigWZ_CMS_jj_VVNP, jj_WZ_sig_frac_CMS_jj_VVNP);

jj_WZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigWZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVHP, jj_WZ_sig_sigma_CMS_jj_qVHP, jj_WZ_sig_alpha_CMS_jj_qVHP, jj_WZ_sig_n_CMS_jj_qVHP);
WZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigWZ_CMS_jj_qVHP, jjCBSigWZ_CMS_jj_qVHP, jj_WZ_sig_frac_CMS_jj_qVHP);

jj_WZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigWZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVLP, jj_WZ_sig_sigma_CMS_jj_qVLP, jj_WZ_sig_alpha_CMS_jj_qVLP, jj_WZ_sig_n_CMS_jj_qVLP);
WZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigWZ_CMS_jj_qVLP, jjCBSigWZ_CMS_jj_qVLP, jj_WZ_sig_frac_CMS_jj_qVLP);


jj_WZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 6500.0];
jj_WZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_WZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_WZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_WZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_WZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigWZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigWZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_WZ_sig_m0_CMS_jj_qVNP, jj_WZ_sig_sigma_CMS_jj_qVNP, jj_WZ_sig_alpha_CMS_jj_qVNP, jj_WZ_sig_n_CMS_jj_qVNP);
WZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigWZ_CMS_jj_qVNP, jjCBSigWZ_CMS_jj_qVNP, jj_WZ_sig_frac_CMS_jj_qVNP);






jj_qW_sig_m0[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma[100, 50.0, 1000.0];
jj_qW_sig_alpha[0.8, 0.5, 3]; 
jj_qW_sig_n[13.0, 0.5, 10]; 
jj_qW_sig_gsigma[100, 50.0, 1000.0];
jj_qW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigqW = Gaussian(mgg13TeV, jj_qW_sig_m0, jj_qW_sig_gsigma);
jjCBSigqW    = CBShape(mgg13TeV, jj_qW_sig_m0, jj_qW_sig_sigma, jj_qW_sig_alpha, jj_qW_sig_n);
qW_jj      = AddPdf(jjGaussSigqW, jjCBSigqW, jj_qW_sig_frac);

jj_qW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigqW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_gsigma_CMS_jj_VVHP);
jjCBSigqW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVHP, jj_qW_sig_sigma_CMS_jj_VVHP, jj_qW_sig_alpha_CMS_jj_VVHP, jj_qW_sig_n_CMS_jj_VVHP);
qW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigqW_CMS_jj_VVHP, jjCBSigqW_CMS_jj_VVHP, jj_qW_sig_frac_CMS_jj_VVHP);

jj_qW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_gsigma_CMS_jj_VVLP);
jjCBSigqW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVLP, jj_qW_sig_sigma_CMS_jj_VVLP, jj_qW_sig_alpha_CMS_jj_VVLP, jj_qW_sig_n_CMS_jj_VVLP);
qW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigqW_CMS_jj_VVLP, jjCBSigqW_CMS_jj_VVLP, jj_qW_sig_frac_CMS_jj_VVLP);

jj_qW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_gsigma_CMS_jj_VVNP);
jjCBSigqW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_VVNP, jj_qW_sig_sigma_CMS_jj_VVNP, jj_qW_sig_alpha_CMS_jj_VVNP, jj_qW_sig_n_CMS_jj_VVNP);
qW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigqW_CMS_jj_VVNP, jjCBSigqW_CMS_jj_VVNP, jj_qW_sig_frac_CMS_jj_VVNP);

jj_qW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_gsigma_CMS_jj_qVHP);
jjCBSigqW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVHP, jj_qW_sig_sigma_CMS_jj_qVHP, jj_qW_sig_alpha_CMS_jj_qVHP, jj_qW_sig_n_CMS_jj_qVHP);
qW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigqW_CMS_jj_qVHP, jjCBSigqW_CMS_jj_qVHP, jj_qW_sig_frac_CMS_jj_qVHP);

jj_qW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_gsigma_CMS_jj_qVLP);
jjCBSigqW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVLP, jj_qW_sig_sigma_CMS_jj_qVLP, jj_qW_sig_alpha_CMS_jj_qVLP, jj_qW_sig_n_CMS_jj_qVLP);
qW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigqW_CMS_jj_qVLP, jjCBSigqW_CMS_jj_qVLP, jj_qW_sig_frac_CMS_jj_qVLP);


jj_qW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 4100.0];
jj_qW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigqW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_gsigma_CMS_jj_qVNP);
jjCBSigqW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_qW_sig_m0_CMS_jj_qVNP, jj_qW_sig_sigma_CMS_jj_qVNP, jj_qW_sig_alpha_CMS_jj_qVNP, jj_qW_sig_n_CMS_jj_qVNP);
qW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigqW_CMS_jj_qVNP, jjCBSigqW_CMS_jj_qVNP, jj_qW_sig_frac_CMS_jj_qVNP);






jj_qZ_sig_m0[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma[100, 50.0, 1000.0];
jj_qZ_sig_alpha[0.8, 0.5, 3]; 
jj_qZ_sig_n[13.0, 0.5, 10]; 
jj_qZ_sig_gsigma[100, 50.0, 1000.0];
jj_qZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigqZ = Gaussian(mgg13TeV, jj_qZ_sig_m0, jj_qZ_sig_gsigma);
jjCBSigqZ    = CBShape(mgg13TeV, jj_qZ_sig_m0, jj_qZ_sig_sigma, jj_qZ_sig_alpha, jj_qZ_sig_n);
qZ_jj      = AddPdf(jjGaussSigqZ, jjCBSigqZ, jj_qZ_sig_frac);

jj_qZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigqZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigqZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVHP, jj_qZ_sig_sigma_CMS_jj_VVHP, jj_qZ_sig_alpha_CMS_jj_VVHP, jj_qZ_sig_n_CMS_jj_VVHP);
qZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigqZ_CMS_jj_VVHP, jjCBSigqZ_CMS_jj_VVHP, jj_qZ_sig_frac_CMS_jj_VVHP);

jj_qZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigqZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVLP, jj_qZ_sig_sigma_CMS_jj_VVLP, jj_qZ_sig_alpha_CMS_jj_VVLP, jj_qZ_sig_n_CMS_jj_VVLP);
qZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigqZ_CMS_jj_VVLP, jjCBSigqZ_CMS_jj_VVLP, jj_qZ_sig_frac_CMS_jj_VVLP);

jj_qZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigqZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_VVNP, jj_qZ_sig_sigma_CMS_jj_VVNP, jj_qZ_sig_alpha_CMS_jj_VVNP, jj_qZ_sig_n_CMS_jj_VVNP);
qZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigqZ_CMS_jj_VVNP, jjCBSigqZ_CMS_jj_VVNP, jj_qZ_sig_frac_CMS_jj_VVNP);

jj_qZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigqZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVHP, jj_qZ_sig_sigma_CMS_jj_qVHP, jj_qZ_sig_alpha_CMS_jj_qVHP, jj_qZ_sig_n_CMS_jj_qVHP);
qZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigqZ_CMS_jj_qVHP, jjCBSigqZ_CMS_jj_qVHP, jj_qZ_sig_frac_CMS_jj_qVHP);

jj_qZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigqZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVLP, jj_qZ_sig_sigma_CMS_jj_qVLP, jj_qZ_sig_alpha_CMS_jj_qVLP, jj_qZ_sig_n_CMS_jj_qVLP);
qZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigqZ_CMS_jj_qVLP, jjCBSigqZ_CMS_jj_qVLP, jj_qZ_sig_frac_CMS_jj_qVLP);


jj_qZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 4100.0];
jj_qZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_qZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_qZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_qZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigqZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigqZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_qZ_sig_m0_CMS_jj_qVNP, jj_qZ_sig_sigma_CMS_jj_qVNP, jj_qZ_sig_alpha_CMS_jj_qVNP, jj_qZ_sig_n_CMS_jj_qVNP);
qZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigqZ_CMS_jj_qVNP, jjCBSigqZ_CMS_jj_qVNP, jj_qZ_sig_frac_CMS_jj_qVNP);





jj_BulkZZ_sig_m0[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha[0.8, 0.5, 3]; 
jj_BulkZZ_sig_n[13.0, 0.5, 10]; 
jj_BulkZZ_sig_gsigma[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac[0.5, 0.3, 1.0];

jjGaussSigBulkZZ = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_gsigma);
jjCBSigBulkZZ    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0, jj_BulkZZ_sig_sigma, jj_BulkZZ_sig_alpha, jj_BulkZZ_sig_n);
BulkZZ_jj      = AddPdf(jjGaussSigBulkZZ, jjCBSigBulkZZ, jj_BulkZZ_sig_frac);

jj_BulkZZ_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigBulkZZ_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkZZ_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVHP, jj_BulkZZ_sig_sigma_CMS_jj_VVHP, jj_BulkZZ_sig_alpha_CMS_jj_VVHP, jj_BulkZZ_sig_n_CMS_jj_VVHP);
BulkZZ_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVHP, jjCBSigBulkZZ_CMS_jj_VVHP, jj_BulkZZ_sig_frac_CMS_jj_VVHP);

jj_BulkZZ_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkZZ_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVLP, jj_BulkZZ_sig_sigma_CMS_jj_VVLP, jj_BulkZZ_sig_alpha_CMS_jj_VVLP, jj_BulkZZ_sig_n_CMS_jj_VVLP);
BulkZZ_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVLP, jjCBSigBulkZZ_CMS_jj_VVLP, jj_BulkZZ_sig_frac_CMS_jj_VVLP);

jj_BulkZZ_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkZZ_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_VVNP, jj_BulkZZ_sig_sigma_CMS_jj_VVNP, jj_BulkZZ_sig_alpha_CMS_jj_VVNP, jj_BulkZZ_sig_n_CMS_jj_VVNP);
BulkZZ_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_VVNP, jjCBSigBulkZZ_CMS_jj_VVNP, jj_BulkZZ_sig_frac_CMS_jj_VVNP);

jj_BulkZZ_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkZZ_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVHP, jj_BulkZZ_sig_sigma_CMS_jj_qVHP, jj_BulkZZ_sig_alpha_CMS_jj_qVHP, jj_BulkZZ_sig_n_CMS_jj_qVHP);
BulkZZ_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVHP, jjCBSigBulkZZ_CMS_jj_qVHP, jj_BulkZZ_sig_frac_CMS_jj_qVHP);

jj_BulkZZ_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkZZ_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVLP, jj_BulkZZ_sig_sigma_CMS_jj_qVLP, jj_BulkZZ_sig_alpha_CMS_jj_qVLP, jj_BulkZZ_sig_n_CMS_jj_qVLP);
BulkZZ_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVLP, jjCBSigBulkZZ_CMS_jj_qVLP, jj_BulkZZ_sig_frac_CMS_jj_qVLP);


jj_BulkZZ_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 6500.0];
jj_BulkZZ_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkZZ_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkZZ_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkZZ_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigBulkZZ_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkZZ_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_BulkZZ_sig_m0_CMS_jj_qVNP, jj_BulkZZ_sig_sigma_CMS_jj_qVNP, jj_BulkZZ_sig_alpha_CMS_jj_qVNP, jj_BulkZZ_sig_n_CMS_jj_qVNP);
BulkZZ_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkZZ_CMS_jj_qVNP, jjCBSigBulkZZ_CMS_jj_qVNP, jj_BulkZZ_sig_frac_CMS_jj_qVNP);






jj_BulkWW_sig_m0[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha[0.8, 0.5, 3]; 
jj_BulkWW_sig_n[13.0, 0.5, 10]; 
jj_BulkWW_sig_gsigma[100, 50.0, 1000.0];
jj_BulkWW_sig_frac[0.5, 0.3, 1.0];

jjGaussSigBulkWW = Gaussian(mgg13TeV, jj_BulkWW_sig_m0, jj_BulkWW_sig_gsigma);
jjCBSigBulkWW    = CBShape(mgg13TeV, jj_BulkWW_sig_m0, jj_BulkWW_sig_sigma, jj_BulkWW_sig_alpha, jj_BulkWW_sig_n);
BulkWW_jj      = AddPdf(jjGaussSigBulkWW, jjCBSigBulkWW, jj_BulkWW_sig_frac);

jj_BulkWW_sig_m0_CMS_jj_VVHP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVHP[0.5, 0.3, 1.0];


jjGaussSigBulkWW_CMS_jj_VVHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_gsigma_CMS_jj_VVHP);
jjCBSigBulkWW_CMS_jj_VVHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVHP, jj_BulkWW_sig_sigma_CMS_jj_VVHP, jj_BulkWW_sig_alpha_CMS_jj_VVHP, jj_BulkWW_sig_n_CMS_jj_VVHP);
BulkWW_jj_CMS_jj_VVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVHP, jjCBSigBulkWW_CMS_jj_VVHP, jj_BulkWW_sig_frac_CMS_jj_VVHP);

jj_BulkWW_sig_m0_CMS_jj_VVLP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVLP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_VVLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_gsigma_CMS_jj_VVLP);
jjCBSigBulkWW_CMS_jj_VVLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVLP, jj_BulkWW_sig_sigma_CMS_jj_VVLP, jj_BulkWW_sig_alpha_CMS_jj_VVLP, jj_BulkWW_sig_n_CMS_jj_VVLP);
BulkWW_jj_CMS_jj_VVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVLP, jjCBSigBulkWW_CMS_jj_VVLP, jj_BulkWW_sig_frac_CMS_jj_VVLP);

jj_BulkWW_sig_m0_CMS_jj_VVNP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_VVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_VVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_VVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_VVNP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_VVNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_gsigma_CMS_jj_VVNP);
jjCBSigBulkWW_CMS_jj_VVNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_VVNP, jj_BulkWW_sig_sigma_CMS_jj_VVNP, jj_BulkWW_sig_alpha_CMS_jj_VVNP, jj_BulkWW_sig_n_CMS_jj_VVNP);
BulkWW_jj_CMS_jj_VVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_VVNP, jjCBSigBulkWW_CMS_jj_VVNP, jj_BulkWW_sig_frac_CMS_jj_VVNP);

jj_BulkWW_sig_m0_CMS_jj_qVHP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVHP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVHP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVHP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVHP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVHP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_gsigma_CMS_jj_qVHP);
jjCBSigBulkWW_CMS_jj_qVHP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVHP, jj_BulkWW_sig_sigma_CMS_jj_qVHP, jj_BulkWW_sig_alpha_CMS_jj_qVHP, jj_BulkWW_sig_n_CMS_jj_qVHP);
BulkWW_jj_CMS_jj_qVHP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVHP, jjCBSigBulkWW_CMS_jj_qVHP, jj_BulkWW_sig_frac_CMS_jj_qVHP);

jj_BulkWW_sig_m0_CMS_jj_qVLP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVLP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVLP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVLP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVLP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVLP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_gsigma_CMS_jj_qVLP);
jjCBSigBulkWW_CMS_jj_qVLP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVLP, jj_BulkWW_sig_sigma_CMS_jj_qVLP, jj_BulkWW_sig_alpha_CMS_jj_qVLP, jj_BulkWW_sig_n_CMS_jj_qVLP);
BulkWW_jj_CMS_jj_qVLP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVLP, jjCBSigBulkWW_CMS_jj_qVLP, jj_BulkWW_sig_frac_CMS_jj_qVLP);


jj_BulkWW_sig_m0_CMS_jj_qVNP[2000.0, 900.0, 6500.0];
jj_BulkWW_sig_sigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_alpha_CMS_jj_qVNP[ 0.8, 0.0, 3.0]; 
jj_BulkWW_sig_n_CMS_jj_qVNP[130, 0.00001, 1000.0]; 
jj_BulkWW_sig_gsigma_CMS_jj_qVNP[100, 50.0, 1000.0];
jj_BulkWW_sig_frac_CMS_jj_qVNP[0.5, 0.3, 1.0];

jjGaussSigBulkWW_CMS_jj_qVNP = Gaussian(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_gsigma_CMS_jj_qVNP);
jjCBSigBulkWW_CMS_jj_qVNP    = CBShape(mgg13TeV, jj_BulkWW_sig_m0_CMS_jj_qVNP, jj_BulkWW_sig_sigma_CMS_jj_qVNP, jj_BulkWW_sig_alpha_CMS_jj_qVNP, jj_BulkWW_sig_n_CMS_jj_qVNP);
BulkWW_jj_CMS_jj_qVNP      = AddPdf(jjGaussSigBulkWW_CMS_jj_qVNP, jjCBSigBulkWW_CMS_jj_qVNP, jj_BulkWW_sig_frac_CMS_jj_qVNP);






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

sqrtS[13000., 13000., 13000.]
