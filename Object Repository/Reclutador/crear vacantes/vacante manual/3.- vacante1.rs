<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3.- vacante1</name>
   <tag></tag>
   <elementGuidId>4604468a-7ec8-4fb4-be35-d4213eb6a3f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.TokenReclu}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;vacantId\&quot;: \&quot;${vacantId}\&quot;,\n    \&quot;idShow\&quot;: \&quot;VA1695076334946\&quot;,\n    \&quot;name\&quot;: \&quot;tester| Automatizacion\&quot;,\n    \&quot;specialty\&quot;: \&quot;Automatizacion\&quot;,\n    \&quot;numbersVacants\&quot;: \&quot;5\&quot;,\n    \&quot;confidential\&quot;: false,\n    \&quot;draftName\&quot;: null,\n    \&quot;functions\&quot;: null,\n    \&quot;peopleCharge\&quot;: null,\n    \&quot;salaryMinimum\&quot;: ${SalarioMin},\n    \&quot;salaryMaximum\&quot;: ${SalarioMax},\n    \&quot;salaryExactly\&quot;: null,\n    \&quot;salaryShow\&quot;: true,\n    \&quot;typeSalary\&quot;: {\n        \&quot;catalogSystemId\&quot;: \&quot;4028e4a986843df5018684517b460003\&quot;,\n        \&quot;name\&quot;: \&quot;BRUTO\&quot;,\n        \&quot;type\&quot;: \&quot;typeSalary\&quot;,\n        \&quot;status\&quot;: true,\n        \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n    },\n    \&quot;commissions\&quot;: false,\n    \&quot;benefits\&quot;: null,\n    \&quot;city\&quot;: {\n        \&quot;cityId\&quot;: \&quot;2c9f936481969f0cccc996a00e090277\&quot;,\n        \&quot;name\&quot;: \&quot;Benito Juárez\&quot;,\n        \&quot;stateCode\&quot;: \&quot;MX-CMX\&quot;,\n        \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n        \&quot;latitude\&quot;: 19.39812,\n        \&quot;longitude\&quot;: -99.1571,\n        \&quot;state\&quot;: {\n            \&quot;stateId\&quot;: \&quot;2c9f936481969f0bbbb996a00e090009\&quot;,\n            \&quot;name\&quot;: \&quot;Ciudad de México\&quot;,\n            \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n            \&quot;fipsCode\&quot;: \&quot;9\&quot;,\n            \&quot;iso2\&quot;: \&quot;MX-CMX\&quot;,\n            \&quot;latitude\&quot;: 19.28333,\n            \&quot;longitude\&quot;: -99.13333,\n            \&quot;country\&quot;: {\n                \&quot;countryId\&quot;: \&quot;2c9f936481969f0aaaa996a00e090001\&quot;,\n                \&quot;capital\&quot;: \&quot;Mexico City\&quot;,\n                \&quot;currency\&quot;: \&quot;MXN\&quot;,\n                \&quot;currencySymbol\&quot;: \&quot;$\&quot;,\n                \&quot;iso2\&quot;: \&quot;MX\&quot;,\n                \&quot;iso3\&quot;: \&quot;MEX\&quot;,\n                \&quot;latitude\&quot;: 19.4284,\n                \&quot;longitude\&quot;: -99.1276,\n                \&quot;name\&quot;: \&quot;Mexico\&quot;,\n                \&quot;nameNative\&quot;: \&quot;México\&quot;,\n                \&quot;phoneCode\&quot;: \&quot;52\&quot;,\n                \&quot;region\&quot;: \&quot;Americas\&quot;,\n                \&quot;subregion\&quot;: \&quot;Central America\&quot;,\n                \&quot;timezones\&quot;: \&quot;[{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Bahia_Banderas\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Cancun\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-18000,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-05:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;EST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Eastern Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Chihuahua\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Hermosillo\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Matamoros\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mazatlan\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Merida\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mexico_City\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Monterrey\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Ojinaga\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Tijuana\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-28800,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-08:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;PST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Pacific Standard Time (North America\\\&quot;}]\&quot;,\n                \&quot;tld\&quot;: \&quot;.mx\&quot;,\n                \&quot;translations\&quot;: \&quot;{\\\&quot;br\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;pt\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;nl\\\&quot;:\\\&quot;Mexico\\\&quot;,\\\&quot;hr\\\&quot;:\\\&quot;Meksiko\\\&quot;,\\\&quot;fa\\\&quot;:\\\&quot;مکزیک\\\&quot;,\\\&quot;de\\\&quot;:\\\&quot;Mexiko\\\&quot;,\\\&quot;es\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;fr\\\&quot;:\\\&quot;Mexique\\\&quot;,\\\&quot;ja\\\&quot;:\\\&quot;メキシコ\\\&quot;,\\\&quot;it\\\&quot;:\\\&quot;Messico\\\&quot;}\&quot;,\n                \&quot;flagCountry\&quot;: \&quot;https://flagcdn.com/w20/mx.png\&quot;\n            }\n        }\n    },\n    \&quot;schedule\&quot;: null,\n    \&quot;contractDuration\&quot;: null,\n    \&quot;createDate\&quot;: \&quot;2023-09-18 17:32:14\&quot;,\n    \&quot;updateDate\&quot;: null,\n    \&quot;publicationDate\&quot;: null,\n    \&quot;academicTitle\&quot;: null,\n    \&quot;mission\&quot;: null,\n    \&quot;allNationality\&quot;: null,\n    \&quot;steps\&quot;: 1,\n    \&quot;levelEducationExclud\&quot;: null,\n    \&quot;statusEducationExclud\&quot;: null,\n    \&quot;modality\&quot;: {\n        \&quot;catalogSystemId\&quot;: \&quot;4028e4a986843df50186845177fa0000\&quot;,\n        \&quot;name\&quot;: \&quot;Presencial\&quot;,\n        \&quot;type\&quot;: \&quot;modalityWork\&quot;,\n        \&quot;status\&quot;: true,\n        \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n    },\n    \&quot;workingDay\&quot;: null,\n    \&quot;status\&quot;: \&quot;BORRADOR\&quot;,\n    \&quot;positionPoints\&quot;: null,\n    \&quot;positionLevelPoints\&quot;: null,\n    \&quot;scholarshipPoints\&quot;: null,\n    \&quot;workExperiencePoints\&quot;: null,\n    \&quot;hardSkillPoints\&quot;: null,\n    \&quot;totalPoints\&quot;: null,\n    \&quot;reasonPause\&quot;: null,\n    \&quot;reasonClose\&quot;: null,\n    \&quot;position\&quot;: {\n        \&quot;positionId\&quot;: \&quot;ff8080818aa9612c018aadbaea9800ea\&quot;,\n        \&quot;position\&quot;: \&quot;${puesto}\&quot;\n    },\n    \&quot;positionId\&quot;: \&quot;ff8080818aa9612c018aadbaea9800ea\&quot;,\n    \&quot;contract\&quot;: null,\n    \&quot;contractId\&quot;: null,\n    \&quot;nationality\&quot;: null,\n    \&quot;nationalityId\&quot;: null,\n    \&quot;education\&quot;: null,\n    \&quot;idEducation\&quot;: null,\n    \&quot;statusEducation\&quot;: null,\n    \&quot;idStatusEducation\&quot;: null,\n    \&quot;typePosition\&quot;: null,\n    \&quot;typePositionId\&quot;: null,\n    \&quot;client\&quot;: {\n        \&quot;clientId\&quot;: \&quot;0000000088c520440188d49176a4005f\&quot;,\n        \&quot;name\&quot;: \&quot;involver\&quot;,\n        \&quot;sector\&quot;: {\n            \&quot;sectorId\&quot;: \&quot;4028808879868679017986ac38b10007\&quot;,\n            \&quot;englishName\&quot;: \&quot;Advertising and graphic arts\&quot;,\n            \&quot;spanishName\&quot;: \&quot;Publicidad y artes gráficas\&quot;,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;sectorId\&quot;: \&quot;4028808879868679017986ac38b10007\&quot;,\n        \&quot;industry\&quot;: {\n            \&quot;industryId\&quot;: \&quot;40288088798a3b0501798a58a24b0038\&quot;,\n            \&quot;englishName\&quot;: \&quot; Advertising agencies\&quot;,\n            \&quot;spanishName\&quot;: \&quot;Agencias de publicidad\&quot;,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;industryId\&quot;: \&quot;40288088798a3b0501798a58a24b0038\&quot;,\n        \&quot;creationDate\&quot;: \&quot;2023-06-19 16:51:23\&quot;,\n        \&quot;typeCompany\&quot;: {\n            \&quot;catalogSystemId\&quot;: \&quot;2c9f906e8684a58a0186942003290004\&quot;,\n            \&quot;name\&quot;: \&quot;NACIONAL\&quot;,\n            \&quot;type\&quot;: \&quot;typeCompany\&quot;,\n            \&quot;status\&quot;: true,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;employees\&quot;: \&quot;DE51A250\&quot;,\n        \&quot;company\&quot;: {\n            \&quot;companyId\&quot;: \&quot;0000000088b55aa80188c58bcd64002e\&quot;,\n            \&quot;company\&quot;: null,\n            \&quot;businessName\&quot;: \&quot;involveMX\&quot;,\n            \&quot;rfc\&quot;: null,\n            \&quot;pathLogo\&quot;: \&quot;https://s3.us-east-1.amazonaws.com/involve-resources-dev-pre/COMPANY/0000000088b55aa80188c58bcd64002e/BUSSINES_LOGO_jpeg\&quot;,\n            \&quot;sector\&quot;: {\n                \&quot;sectorId\&quot;: \&quot;4028808879868679017986ac3c45000a\&quot;,\n                \&quot;englishName\&quot;: \&quot;Technology and telecommunications\&quot;,\n                \&quot;spanishName\&quot;: \&quot;Tecnología y telecomunicaciones\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;industry\&quot;: {\n                \&quot;industryId\&quot;: \&quot;40288088798a3b0501798a58ca500059\&quot;,\n                \&quot;englishName\&quot;: \&quot; Software\&quot;,\n                \&quot;spanishName\&quot;: \&quot;Software\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;typeCompany\&quot;: {\n                \&quot;catalogSystemId\&quot;: \&quot;2c9f906e8684a58a0186942003320005\&quot;,\n                \&quot;name\&quot;: \&quot;INTERNACIONAL\&quot;,\n                \&quot;type\&quot;: \&quot;typeCompany\&quot;,\n                \&quot;status\&quot;: true,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;companySize\&quot;: \&quot;DE11A50\&quot;,\n            \&quot;country\&quot;: \&quot;MX\&quot;,\n            \&quot;colorTextButton\&quot;: null,\n            \&quot;colorText\&quot;: null,\n            \&quot;pathLogoEmail\&quot;: \&quot;https://involve-resources.s3.amazonaws.com/res-involve/company-avatar.png\&quot;,\n            \&quot;colorButton\&quot;: null,\n            \&quot;website\&quot;: null,\n            \&quot;description\&quot;: null,\n            \&quot;totalCoins\&quot;: 2,\n            \&quot;facturapiCustomerId\&quot;: \&quot;648cc09afef7e36777c0ed22\&quot;,\n            \&quot;facturapiUseCfdi\&quot;: \&quot;D01\&quot;,\n            \&quot;vacancyApprove\&quot;: \&quot;NEVER\&quot;,\n            \&quot;invoiceNotification\&quot;: true,\n            \&quot;legalName\&quot;: null,\n            \&quot;address\&quot;: null,\n            \&quot;zipCode\&quot;: null,\n            \&quot;invoiceCounter\&quot;: null\n        },\n        \&quot;companyId\&quot;: \&quot;0000000088b55aa80188c58bcd64002e\&quot;,\n        \&quot;isMine\&quot;: false,\n        \&quot;isActive\&quot;: true,\n        \&quot;webSite\&quot;: null,\n        \&quot;description\&quot;: null,\n        \&quot;pathLogo\&quot;: null,\n        \&quot;recruiter\&quot;: null,\n        \&quot;recruiterId\&quot;: null\n    },\n    \&quot;clientId\&quot;: \&quot;0000000088c520440188d49176a4005f\&quot;,\n    \&quot;company\&quot;: {\n        \&quot;companyId\&quot;: \&quot;0000000088b55aa80188c58bcd64002e\&quot;,\n        \&quot;company\&quot;: null,\n        \&quot;businessName\&quot;: \&quot;involveMX\&quot;,\n        \&quot;rfc\&quot;: null,\n        \&quot;pathLogo\&quot;: \&quot;https://s3.us-east-1.amazonaws.com/involve-resources-dev-pre/COMPANY/0000000088b55aa80188c58bcd64002e/BUSSINES_LOGO_jpeg\&quot;,\n        \&quot;sector\&quot;: {\n            \&quot;sectorId\&quot;: \&quot;4028808879868679017986ac3c45000a\&quot;,\n            \&quot;englishName\&quot;: \&quot;Technology and telecommunications\&quot;,\n            \&quot;spanishName\&quot;: \&quot;Tecnología y telecomunicaciones\&quot;,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;industry\&quot;: {\n            \&quot;industryId\&quot;: \&quot;40288088798a3b0501798a58ca500059\&quot;,\n            \&quot;englishName\&quot;: \&quot; Software\&quot;,\n            \&quot;spanishName\&quot;: \&quot;Software\&quot;,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;typeCompany\&quot;: {\n            \&quot;catalogSystemId\&quot;: \&quot;2c9f906e8684a58a0186942003320005\&quot;,\n            \&quot;name\&quot;: \&quot;INTERNACIONAL\&quot;,\n            \&quot;type\&quot;: \&quot;typeCompany\&quot;,\n            \&quot;status\&quot;: true,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        },\n        \&quot;companySize\&quot;: \&quot;DE11A50\&quot;,\n        \&quot;country\&quot;: \&quot;MX\&quot;,\n        \&quot;colorTextButton\&quot;: null,\n        \&quot;colorText\&quot;: null,\n        \&quot;pathLogoEmail\&quot;: \&quot;https://involve-resources.s3.amazonaws.com/res-involve/company-avatar.png\&quot;,\n        \&quot;colorButton\&quot;: null,\n        \&quot;website\&quot;: null,\n        \&quot;description\&quot;: null,\n        \&quot;totalCoins\&quot;: 2,\n        \&quot;facturapiCustomerId\&quot;: \&quot;648cc09afef7e36777c0ed22\&quot;,\n        \&quot;facturapiUseCfdi\&quot;: \&quot;D01\&quot;,\n        \&quot;vacancyApprove\&quot;: \&quot;NEVER\&quot;,\n        \&quot;invoiceNotification\&quot;: true,\n        \&quot;legalName\&quot;: null,\n        \&quot;address\&quot;: null,\n        \&quot;zipCode\&quot;: null,\n        \&quot;invoiceCounter\&quot;: null\n    },\n    \&quot;companyId\&quot;: \&quot;0000000088b55aa80188c58bcd64002e\&quot;,\n    \&quot;otherBenefits\&quot;: null,\n    \&quot;nextRevision\&quot;: null,\n    \&quot;lastRevision\&quot;: null,\n    \&quot;usersDelete\&quot;: 0,\n    \&quot;nuevo\&quot;: 0,\n    \&quot;video\&quot;: 0,\n    \&quot;finalista\&quot;: 0,\n    \&quot;contratado\&quot;: 0,\n    \&quot;candidateRejects\&quot;: 0,\n    \&quot;vacantIdOriginal\&quot;: null,\n    \&quot;vacantOriginalName\&quot;: null,\n    \&quot;periodicitySalary\&quot;: {\n        \&quot;catalogSystemId\&quot;: \&quot;0000000088d5e9020188dfc7be19001d\&quot;,\n        \&quot;name\&quot;: \&quot;Mes\&quot;,\n        \&quot;type\&quot;: \&quot;periodicitySalary\&quot;,\n        \&quot;status\&quot;: true,\n        \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n    },\n    \&quot;keySystem\&quot;: \&quot;MEX\&quot;,\n    \&quot;matchMin\&quot;: 60,\n    \&quot;matchMax\&quot;: 100,\n    \&quot;capacity\&quot;: 10,\n    \&quot;areasVacant\&quot;: null,\n    \&quot;recruiters\&quot;: null,\n    \&quot;hardSkillVacant\&quot;: null,\n    \&quot;institutionVacant\&quot;: null,\n    \&quot;languageVacant\&quot;: null,\n    \&quot;softSkillVacant\&quot;: null,\n    \&quot;specialityVacant\&quot;: null,\n    \&quot;benefitsInvolve\&quot;: [\n        {\n            \&quot;vacantBenefitId\&quot;: \&quot;ff8080818aa9612c018aaa6c3d7800d7\&quot;,\n            \&quot;benefit\&quot;: {\n                \&quot;benefitsId\&quot;: \&quot;ff8081817b837f74017b838148a00000\&quot;,\n                \&quot;description\&quot;: \&quot;Prestaciones de ley (IMSS, vacaciones, aguinaldo)\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;benefitId\&quot;: \&quot;ff8081817b837f74017b838148a00000\&quot;,\n            \&quot;benefitOther\&quot;: null,\n            \&quot;otherBenefitsId\&quot;: null,\n            \&quot;vacantId\&quot;: \&quot;${vacantId}\&quot;\n        },\n        {\n            \&quot;vacantBenefitId\&quot;: \&quot;ff8080818aa9612c018aaa6c3d8600d8\&quot;,\n            \&quot;benefit\&quot;: {\n                \&quot;benefitsId\&quot;: \&quot;ff8081817b837f74017b838225670001\&quot;,\n                \&quot;description\&quot;: \&quot;Bonos y Comisiones\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;benefitId\&quot;: \&quot;ff8081817b837f74017b838225670001\&quot;,\n            \&quot;benefitOther\&quot;: null,\n            \&quot;otherBenefitsId\&quot;: null,\n            \&quot;vacantId\&quot;: \&quot;${vacantId}\&quot;\n        },\n        {\n            \&quot;vacantBenefitId\&quot;: \&quot;ff8080818aa9612c018aaa6c3d9200d9\&quot;,\n            \&quot;benefit\&quot;: {\n                \&quot;benefitsId\&quot;: \&quot;ff8081817b837f74017b838776f70002\&quot;,\n                \&quot;description\&quot;: \&quot;Contratación directa (100% Nómina)\&quot;,\n                \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n            },\n            \&quot;benefitId\&quot;: \&quot;ff8081817b837f74017b838776f70002\&quot;,\n            \&quot;benefitOther\&quot;: null,\n            \&quot;otherBenefitsId\&quot;: null,\n            \&quot;vacantId\&quot;: \&quot;${vacantId}\&quot;\n        }\n    ],\n    \&quot;questions\&quot;: null,\n    \&quot;listVacantPsychometricTestInvolve\&quot;: null\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2cdc8d48-e45a-4d9c-9a6f-c0790cdd858e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenReclu}</value>
      <webElementGuid>348f88b4-739d-4389-81e5-49e2ace67aa3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/vacancy/management</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.puesto</defaultValue>
      <description></description>
      <id>f4424203-4cb3-4dcb-a585-6bf4acb8be74</id>
      <masked>false</masked>
      <name>puesto</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>a5119345-8959-4dd3-9fa2-cec048ee11d2</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SalarioMin</defaultValue>
      <description></description>
      <id>6821b0d7-386f-483f-b50b-45b522775888</id>
      <masked>false</masked>
      <name>SalarioMin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SalarioMax</defaultValue>
      <description></description>
      <id>89e31f01-1542-4cbb-9b76-e9889c746da1</id>
      <masked>false</masked>
      <name>SalarioMax</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.salarioShow</defaultValue>
      <description></description>
      <id>6fe43721-53f7-4a94-8f48-6c68c6aa5b4b</id>
      <masked>false</masked>
      <name>salarioShow</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.confidencial</defaultValue>
      <description></description>
      <id>ffbeffac-4dfe-494c-bfc0-3c0fb977c7e5</id>
      <masked>false</masked>
      <name>confidencial</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.clientId</defaultValue>
      <description></description>
      <id>53310ec1-2ed9-45c1-88cc-30a662ee7b42</id>
      <masked>false</masked>
      <name>clientId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.vacanteid</defaultValue>
      <description></description>
      <id>b90bbe03-5295-401e-b16b-8294bfb7481e</id>
      <masked>false</masked>
      <name>vacantId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.empresa</defaultValue>
      <description></description>
      <id>e7901d22-cd5a-45b9-8457-6d7618b199b3</id>
      <masked>false</masked>
      <name>companyId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
