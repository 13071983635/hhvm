/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */
#[cfg(test)]
mod tests {
    use html_entities::decoder::*;
    use pretty_assertions::assert_eq;

    fn helper(x: u32) -> Vec<u8> {
        html_entities::utf32_to_utf8_alloc(x)
    }

    #[test]
    fn test() {
        assert_eq!(&helper(160)[..], decode(b"nbsp").unwrap());
        assert_eq!(&helper(161)[..], decode(b"iexcl").unwrap());
        assert_eq!(&helper(162)[..], decode(b"cent").unwrap());
        assert_eq!(&helper(163)[..], decode(b"pound").unwrap());
        assert_eq!(&helper(164)[..], decode(b"curren").unwrap());
        assert_eq!(&helper(165)[..], decode(b"yen").unwrap());
        assert_eq!(&helper(166)[..], decode(b"brvbar").unwrap());
        assert_eq!(&helper(167)[..], decode(b"sect").unwrap());
        assert_eq!(&helper(168)[..], decode(b"uml").unwrap());
        assert_eq!(&helper(169)[..], decode(b"copy").unwrap());
        assert_eq!(&helper(170)[..], decode(b"ordf").unwrap());
        assert_eq!(&helper(171)[..], decode(b"laquo").unwrap());
        assert_eq!(&helper(172)[..], decode(b"not").unwrap());
        assert_eq!(&helper(173)[..], decode(b"shy").unwrap());
        assert_eq!(&helper(174)[..], decode(b"reg").unwrap());
        assert_eq!(&helper(175)[..], decode(b"macr").unwrap());
        assert_eq!(&helper(176)[..], decode(b"deg").unwrap());
        assert_eq!(&helper(177)[..], decode(b"plusmn").unwrap());
        assert_eq!(&helper(178)[..], decode(b"sup2").unwrap());
        assert_eq!(&helper(179)[..], decode(b"sup3").unwrap());
        assert_eq!(&helper(180)[..], decode(b"acute").unwrap());
        assert_eq!(&helper(181)[..], decode(b"micro").unwrap());
        assert_eq!(&helper(182)[..], decode(b"para").unwrap());
        assert_eq!(&helper(183)[..], decode(b"middot").unwrap());
        assert_eq!(&helper(184)[..], decode(b"cedil").unwrap());
        assert_eq!(&helper(185)[..], decode(b"sup1").unwrap());
        assert_eq!(&helper(186)[..], decode(b"ordm").unwrap());
        assert_eq!(&helper(187)[..], decode(b"raquo").unwrap());
        assert_eq!(&helper(188)[..], decode(b"frac14").unwrap());
        assert_eq!(&helper(189)[..], decode(b"frac12").unwrap());
        assert_eq!(&helper(190)[..], decode(b"frac34").unwrap());
        assert_eq!(&helper(191)[..], decode(b"iquest").unwrap());
        assert_eq!(&helper(192)[..], decode(b"Agrave").unwrap());
        assert_eq!(&helper(193)[..], decode(b"Aacute").unwrap());
        assert_eq!(&helper(194)[..], decode(b"Acirc").unwrap());
        assert_eq!(&helper(195)[..], decode(b"Atilde").unwrap());
        assert_eq!(&helper(196)[..], decode(b"Auml").unwrap());
        assert_eq!(&helper(197)[..], decode(b"Aring").unwrap());
        assert_eq!(&helper(198)[..], decode(b"AElig").unwrap());
        assert_eq!(&helper(199)[..], decode(b"Ccedil").unwrap());
        assert_eq!(&helper(200)[..], decode(b"Egrave").unwrap());
        assert_eq!(&helper(201)[..], decode(b"Eacute").unwrap());
        assert_eq!(&helper(202)[..], decode(b"Ecirc").unwrap());
        assert_eq!(&helper(203)[..], decode(b"Euml").unwrap());
        assert_eq!(&helper(204)[..], decode(b"Igrave").unwrap());
        assert_eq!(&helper(205)[..], decode(b"Iacute").unwrap());
        assert_eq!(&helper(206)[..], decode(b"Icirc").unwrap());
        assert_eq!(&helper(207)[..], decode(b"Iuml").unwrap());
        assert_eq!(&helper(208)[..], decode(b"ETH").unwrap());
        assert_eq!(&helper(209)[..], decode(b"Ntilde").unwrap());
        assert_eq!(&helper(210)[..], decode(b"Ograve").unwrap());
        assert_eq!(&helper(211)[..], decode(b"Oacute").unwrap());
        assert_eq!(&helper(212)[..], decode(b"Ocirc").unwrap());
        assert_eq!(&helper(213)[..], decode(b"Otilde").unwrap());
        assert_eq!(&helper(214)[..], decode(b"Ouml").unwrap());
        assert_eq!(&helper(215)[..], decode(b"times").unwrap());
        assert_eq!(&helper(216)[..], decode(b"Oslash").unwrap());
        assert_eq!(&helper(217)[..], decode(b"Ugrave").unwrap());
        assert_eq!(&helper(218)[..], decode(b"Uacute").unwrap());
        assert_eq!(&helper(219)[..], decode(b"Ucirc").unwrap());
        assert_eq!(&helper(220)[..], decode(b"Uuml").unwrap());
        assert_eq!(&helper(221)[..], decode(b"Yacute").unwrap());
        assert_eq!(&helper(222)[..], decode(b"THORN").unwrap());
        assert_eq!(&helper(223)[..], decode(b"szlig").unwrap());
        assert_eq!(&helper(224)[..], decode(b"agrave").unwrap());
        assert_eq!(&helper(225)[..], decode(b"aacute").unwrap());
        assert_eq!(&helper(226)[..], decode(b"acirc").unwrap());
        assert_eq!(&helper(227)[..], decode(b"atilde").unwrap());
        assert_eq!(&helper(228)[..], decode(b"auml").unwrap());
        assert_eq!(&helper(229)[..], decode(b"aring").unwrap());
        assert_eq!(&helper(230)[..], decode(b"aelig").unwrap());
        assert_eq!(&helper(231)[..], decode(b"ccedil").unwrap());
        assert_eq!(&helper(232)[..], decode(b"egrave").unwrap());
        assert_eq!(&helper(233)[..], decode(b"eacute").unwrap());
        assert_eq!(&helper(234)[..], decode(b"ecirc").unwrap());
        assert_eq!(&helper(235)[..], decode(b"euml").unwrap());
        assert_eq!(&helper(236)[..], decode(b"igrave").unwrap());
        assert_eq!(&helper(237)[..], decode(b"iacute").unwrap());
        assert_eq!(&helper(238)[..], decode(b"icirc").unwrap());
        assert_eq!(&helper(239)[..], decode(b"iuml").unwrap());
        assert_eq!(&helper(240)[..], decode(b"eth").unwrap());
        assert_eq!(&helper(241)[..], decode(b"ntilde").unwrap());
        assert_eq!(&helper(242)[..], decode(b"ograve").unwrap());
        assert_eq!(&helper(243)[..], decode(b"oacute").unwrap());
        assert_eq!(&helper(244)[..], decode(b"ocirc").unwrap());
        assert_eq!(&helper(245)[..], decode(b"otilde").unwrap());
        assert_eq!(&helper(246)[..], decode(b"ouml").unwrap());
        assert_eq!(&helper(247)[..], decode(b"divide").unwrap());
        assert_eq!(&helper(248)[..], decode(b"oslash").unwrap());
        assert_eq!(&helper(249)[..], decode(b"ugrave").unwrap());
        assert_eq!(&helper(250)[..], decode(b"uacute").unwrap());
        assert_eq!(&helper(251)[..], decode(b"ucirc").unwrap());
        assert_eq!(&helper(252)[..], decode(b"uuml").unwrap());
        assert_eq!(&helper(253)[..], decode(b"yacute").unwrap());
        assert_eq!(&helper(254)[..], decode(b"thorn").unwrap());
        assert_eq!(&helper(255)[..], decode(b"yuml").unwrap());
        assert_eq!(&helper(338)[..], decode(b"OElig").unwrap());
        assert_eq!(&helper(339)[..], decode(b"oelig").unwrap());
        assert_eq!(&helper(352)[..], decode(b"Scaron").unwrap());
        assert_eq!(&helper(353)[..], decode(b"scaron").unwrap());
        assert_eq!(&helper(376)[..], decode(b"Yuml").unwrap());
        assert_eq!(&helper(402)[..], decode(b"fnof").unwrap());
        assert_eq!(&helper(710)[..], decode(b"circ").unwrap());
        assert_eq!(&helper(732)[..], decode(b"tilde").unwrap());
        assert_eq!(&helper(913)[..], decode(b"Alpha").unwrap());
        assert_eq!(&helper(914)[..], decode(b"Beta").unwrap());
        assert_eq!(&helper(915)[..], decode(b"Gamma").unwrap());
        assert_eq!(&helper(916)[..], decode(b"Delta").unwrap());
        assert_eq!(&helper(917)[..], decode(b"Epsilon").unwrap());
        assert_eq!(&helper(918)[..], decode(b"Zeta").unwrap());
        assert_eq!(&helper(919)[..], decode(b"Eta").unwrap());
        assert_eq!(&helper(920)[..], decode(b"Theta").unwrap());
        assert_eq!(&helper(921)[..], decode(b"Iota").unwrap());
        assert_eq!(&helper(922)[..], decode(b"Kappa").unwrap());
        assert_eq!(&helper(923)[..], decode(b"Lambda").unwrap());
        assert_eq!(&helper(924)[..], decode(b"Mu").unwrap());
        assert_eq!(&helper(925)[..], decode(b"Nu").unwrap());
        assert_eq!(&helper(926)[..], decode(b"Xi").unwrap());
        assert_eq!(&helper(927)[..], decode(b"Omicron").unwrap());
        assert_eq!(&helper(928)[..], decode(b"Pi").unwrap());
        assert_eq!(&helper(929)[..], decode(b"Rho").unwrap());
        assert_eq!(&helper(931)[..], decode(b"Sigma").unwrap());
        assert_eq!(&helper(932)[..], decode(b"Tau").unwrap());
        assert_eq!(&helper(933)[..], decode(b"Upsilon").unwrap());
        assert_eq!(&helper(934)[..], decode(b"Phi").unwrap());
        assert_eq!(&helper(935)[..], decode(b"Chi").unwrap());
        assert_eq!(&helper(936)[..], decode(b"Psi").unwrap());
        assert_eq!(&helper(937)[..], decode(b"Omega").unwrap());
        assert_eq!(&helper(945)[..], decode(b"alpha").unwrap());
        assert_eq!(&helper(946)[..], decode(b"beta").unwrap());
        assert_eq!(&helper(947)[..], decode(b"gamma").unwrap());
        assert_eq!(&helper(948)[..], decode(b"delta").unwrap());
        assert_eq!(&helper(949)[..], decode(b"epsilon").unwrap());
        assert_eq!(&helper(950)[..], decode(b"zeta").unwrap());
        assert_eq!(&helper(951)[..], decode(b"eta").unwrap());
        assert_eq!(&helper(952)[..], decode(b"theta").unwrap());
        assert_eq!(&helper(953)[..], decode(b"iota").unwrap());
        assert_eq!(&helper(954)[..], decode(b"kappa").unwrap());
        assert_eq!(&helper(955)[..], decode(b"lambda").unwrap());
        assert_eq!(&helper(956)[..], decode(b"mu").unwrap());
        assert_eq!(&helper(957)[..], decode(b"nu").unwrap());
        assert_eq!(&helper(958)[..], decode(b"xi").unwrap());
        assert_eq!(&helper(959)[..], decode(b"omicron").unwrap());
        assert_eq!(&helper(960)[..], decode(b"pi").unwrap());
        assert_eq!(&helper(961)[..], decode(b"rho").unwrap());
        assert_eq!(&helper(962)[..], decode(b"sigmaf").unwrap());
        assert_eq!(&helper(963)[..], decode(b"sigma").unwrap());
        assert_eq!(&helper(964)[..], decode(b"tau").unwrap());
        assert_eq!(&helper(965)[..], decode(b"upsilon").unwrap());
        assert_eq!(&helper(966)[..], decode(b"phi").unwrap());
        assert_eq!(&helper(967)[..], decode(b"chi").unwrap());
        assert_eq!(&helper(968)[..], decode(b"psi").unwrap());
        assert_eq!(&helper(969)[..], decode(b"omega").unwrap());
        assert_eq!(&helper(977)[..], decode(b"thetasym").unwrap());
        assert_eq!(&helper(978)[..], decode(b"upsih").unwrap());
        assert_eq!(&helper(982)[..], decode(b"piv").unwrap());
        assert_eq!(&helper(8194)[..], decode(b"ensp").unwrap());
        assert_eq!(&helper(8195)[..], decode(b"emsp").unwrap());
        assert_eq!(&helper(8201)[..], decode(b"thinsp").unwrap());
        assert_eq!(&helper(8204)[..], decode(b"zwnj").unwrap());
        assert_eq!(&helper(8205)[..], decode(b"zwj").unwrap());
        assert_eq!(&helper(8206)[..], decode(b"lrm").unwrap());
        assert_eq!(&helper(8207)[..], decode(b"rlm").unwrap());
        assert_eq!(&helper(8211)[..], decode(b"ndash").unwrap());
        assert_eq!(&helper(8212)[..], decode(b"mdash").unwrap());
        assert_eq!(&helper(8216)[..], decode(b"lsquo").unwrap());
        assert_eq!(&helper(8217)[..], decode(b"rsquo").unwrap());
        assert_eq!(&helper(8218)[..], decode(b"sbquo").unwrap());
        assert_eq!(&helper(8220)[..], decode(b"ldquo").unwrap());
        assert_eq!(&helper(8221)[..], decode(b"rdquo").unwrap());
        assert_eq!(&helper(8222)[..], decode(b"bdquo").unwrap());
        assert_eq!(&helper(8224)[..], decode(b"dagger").unwrap());
        assert_eq!(&helper(8225)[..], decode(b"Dagger").unwrap());
        assert_eq!(&helper(8226)[..], decode(b"bull").unwrap());
        assert_eq!(&helper(8230)[..], decode(b"hellip").unwrap());
        assert_eq!(&helper(8240)[..], decode(b"permil").unwrap());
        assert_eq!(&helper(8242)[..], decode(b"prime").unwrap());
        assert_eq!(&helper(8243)[..], decode(b"Prime").unwrap());
        assert_eq!(&helper(8249)[..], decode(b"lsaquo").unwrap());
        assert_eq!(&helper(8250)[..], decode(b"rsaquo").unwrap());
        assert_eq!(&helper(8254)[..], decode(b"oline").unwrap());
        assert_eq!(&helper(8260)[..], decode(b"frasl").unwrap());
        assert_eq!(&helper(8364)[..], decode(b"euro").unwrap());
        assert_eq!(&helper(8465)[..], decode(b"image").unwrap());
        assert_eq!(&helper(8472)[..], decode(b"weierp").unwrap());
        assert_eq!(&helper(8476)[..], decode(b"real").unwrap());
        assert_eq!(&helper(8482)[..], decode(b"trade").unwrap());
        assert_eq!(&helper(8501)[..], decode(b"alefsym").unwrap());
        assert_eq!(&helper(8592)[..], decode(b"larr").unwrap());
        assert_eq!(&helper(8593)[..], decode(b"uarr").unwrap());
        assert_eq!(&helper(8594)[..], decode(b"rarr").unwrap());
        assert_eq!(&helper(8595)[..], decode(b"darr").unwrap());
        assert_eq!(&helper(8596)[..], decode(b"harr").unwrap());
        assert_eq!(&helper(8629)[..], decode(b"crarr").unwrap());
        assert_eq!(&helper(8656)[..], decode(b"lArr").unwrap());
        assert_eq!(&helper(8657)[..], decode(b"uArr").unwrap());
        assert_eq!(&helper(8658)[..], decode(b"rArr").unwrap());
        assert_eq!(&helper(8659)[..], decode(b"dArr").unwrap());
        assert_eq!(&helper(8660)[..], decode(b"hArr").unwrap());
        assert_eq!(&helper(8661)[..], decode(b"vArr").unwrap());
        assert_eq!(&helper(8666)[..], decode(b"lAarr").unwrap());
        assert_eq!(&helper(8667)[..], decode(b"rAarr").unwrap());
        assert_eq!(&helper(8669)[..], decode(b"rarrw").unwrap());
        assert_eq!(&helper(8704)[..], decode(b"forall").unwrap());
        assert_eq!(&helper(8705)[..], decode(b"comp").unwrap());
        assert_eq!(&helper(8706)[..], decode(b"part").unwrap());
        assert_eq!(&helper(8707)[..], decode(b"exist").unwrap());
        assert_eq!(&helper(8708)[..], decode(b"nexist").unwrap());
        assert_eq!(&helper(8709)[..], decode(b"empty").unwrap());
        assert_eq!(&helper(8711)[..], decode(b"nabla").unwrap());
        assert_eq!(&helper(8712)[..], decode(b"isin").unwrap());
        assert_eq!(&helper(8713)[..], decode(b"notin").unwrap());
        assert_eq!(&helper(8714)[..], decode(b"epsis").unwrap());
        assert_eq!(&helper(8715)[..], decode(b"ni").unwrap());
        assert_eq!(&helper(8716)[..], decode(b"notni").unwrap());
        assert_eq!(&helper(8717)[..], decode(b"bepsi").unwrap());
        assert_eq!(&helper(8719)[..], decode(b"prod").unwrap());
        assert_eq!(&helper(8720)[..], decode(b"coprod").unwrap());
        assert_eq!(&helper(8721)[..], decode(b"sum").unwrap());
        assert_eq!(&helper(8722)[..], decode(b"minus").unwrap());
        assert_eq!(&helper(8723)[..], decode(b"mnplus").unwrap());
        assert_eq!(&helper(8724)[..], decode(b"plusdo").unwrap());
        assert_eq!(&helper(8726)[..], decode(b"setmn").unwrap());
        assert_eq!(&helper(8727)[..], decode(b"lowast").unwrap());
        assert_eq!(&helper(8728)[..], decode(b"compfn").unwrap());
        assert_eq!(&helper(8730)[..], decode(b"radic").unwrap());
        assert_eq!(&helper(8733)[..], decode(b"prop").unwrap());
        assert_eq!(&helper(8734)[..], decode(b"infin").unwrap());
        assert_eq!(&helper(8735)[..], decode(b"ang90").unwrap());
        assert_eq!(&helper(8736)[..], decode(b"ang").unwrap());
        assert_eq!(&helper(8737)[..], decode(b"angmsd").unwrap());
        assert_eq!(&helper(8738)[..], decode(b"angsph").unwrap());
        assert_eq!(&helper(8739)[..], decode(b"mid").unwrap());
        assert_eq!(&helper(8740)[..], decode(b"nmid").unwrap());
        assert_eq!(&helper(8741)[..], decode(b"par").unwrap());
        assert_eq!(&helper(8742)[..], decode(b"npar").unwrap());
        assert_eq!(&helper(8743)[..], decode(b"and").unwrap());
        assert_eq!(&helper(8744)[..], decode(b"or").unwrap());
        assert_eq!(&helper(8745)[..], decode(b"cap").unwrap());
        assert_eq!(&helper(8746)[..], decode(b"cup").unwrap());
        assert_eq!(&helper(8747)[..], decode(b"int").unwrap());
        assert_eq!(&helper(8750)[..], decode(b"conint").unwrap());
        assert_eq!(&helper(8756)[..], decode(b"there4").unwrap());
        assert_eq!(&helper(8757)[..], decode(b"becaus").unwrap());
        assert_eq!(&helper(8764)[..], decode(b"sim").unwrap());
        assert_eq!(&helper(8765)[..], decode(b"bsim").unwrap());
        assert_eq!(&helper(8768)[..], decode(b"wreath").unwrap());
        assert_eq!(&helper(8769)[..], decode(b"nsim").unwrap());
        assert_eq!(&helper(8771)[..], decode(b"sime").unwrap());
        assert_eq!(&helper(8772)[..], decode(b"nsime").unwrap());
        assert_eq!(&helper(8773)[..], decode(b"cong").unwrap());
        assert_eq!(&helper(8775)[..], decode(b"ncong").unwrap());
        assert_eq!(&helper(8776)[..], decode(b"asymp").unwrap());
        assert_eq!(&helper(8777)[..], decode(b"nap").unwrap());
        assert_eq!(&helper(8778)[..], decode(b"ape").unwrap());
        assert_eq!(&helper(8780)[..], decode(b"bcong").unwrap());
        assert_eq!(&helper(8781)[..], decode(b"asympeq").unwrap());
        assert_eq!(&helper(8782)[..], decode(b"bump").unwrap());
        assert_eq!(&helper(8783)[..], decode(b"bumpe").unwrap());
        assert_eq!(&helper(8800)[..], decode(b"ne").unwrap());
        assert_eq!(&helper(8801)[..], decode(b"equiv").unwrap());
        assert_eq!(&helper(8804)[..], decode(b"le").unwrap());
        assert_eq!(&helper(8805)[..], decode(b"ge").unwrap());
        assert_eq!(&helper(8806)[..], decode(b"lE").unwrap());
        assert_eq!(&helper(8807)[..], decode(b"gE").unwrap());
        assert_eq!(&helper(8808)[..], decode(b"lnE").unwrap());
        assert_eq!(&helper(8809)[..], decode(b"gnE").unwrap());
        assert_eq!(&helper(8810)[..], decode(b"Lt").unwrap());
        assert_eq!(&helper(8811)[..], decode(b"Gt").unwrap());
        assert_eq!(&helper(8812)[..], decode(b"twixt").unwrap());
        assert_eq!(&helper(8814)[..], decode(b"nlt").unwrap());
        assert_eq!(&helper(8815)[..], decode(b"ngt").unwrap());
        assert_eq!(&helper(8816)[..], decode(b"nles").unwrap());
        assert_eq!(&helper(8817)[..], decode(b"nges").unwrap());
        assert_eq!(&helper(8818)[..], decode(b"lsim").unwrap());
        assert_eq!(&helper(8819)[..], decode(b"gsim").unwrap());
        assert_eq!(&helper(8822)[..], decode(b"lg").unwrap());
        assert_eq!(&helper(8823)[..], decode(b"gl").unwrap());
        assert_eq!(&helper(8826)[..], decode(b"pr").unwrap());
        assert_eq!(&helper(8827)[..], decode(b"sc").unwrap());
        assert_eq!(&helper(8828)[..], decode(b"cupre").unwrap());
        assert_eq!(&helper(8829)[..], decode(b"sscue").unwrap());
        assert_eq!(&helper(8830)[..], decode(b"prsim").unwrap());
        assert_eq!(&helper(8831)[..], decode(b"scsim").unwrap());
        assert_eq!(&helper(8832)[..], decode(b"npr").unwrap());
        assert_eq!(&helper(8833)[..], decode(b"nsc").unwrap());
        assert_eq!(&helper(8834)[..], decode(b"sub").unwrap());
        assert_eq!(&helper(8835)[..], decode(b"sup").unwrap());
        assert_eq!(&helper(8836)[..], decode(b"nsub").unwrap());
        assert_eq!(&helper(8837)[..], decode(b"nsup").unwrap());
        assert_eq!(&helper(8838)[..], decode(b"sube").unwrap());
        assert_eq!(&helper(8839)[..], decode(b"supe").unwrap());
        assert_eq!(&helper(8853)[..], decode(b"oplus").unwrap());
        assert_eq!(&helper(8855)[..], decode(b"otimes").unwrap());
        assert_eq!(&helper(8869)[..], decode(b"perp").unwrap());
        assert_eq!(&helper(8901)[..], decode(b"sdot").unwrap());
        assert_eq!(&helper(8968)[..], decode(b"lceil").unwrap());
        assert_eq!(&helper(8969)[..], decode(b"rceil").unwrap());
        assert_eq!(&helper(8970)[..], decode(b"lfloor").unwrap());
        assert_eq!(&helper(8971)[..], decode(b"rfloor").unwrap());
        assert_eq!(&helper(9001)[..], decode(b"lang").unwrap());
        assert_eq!(&helper(9002)[..], decode(b"rang").unwrap());
        assert_eq!(&helper(9674)[..], decode(b"loz").unwrap());
        assert_eq!(&helper(9824)[..], decode(b"spades").unwrap());
        assert_eq!(&helper(9827)[..], decode(b"clubs").unwrap());
        assert_eq!(&helper(9829)[..], decode(b"hearts").unwrap());
        assert_eq!(&helper(9830)[..], decode(b"diams").unwrap());

    }
}