<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>vacante5</name>
   <tag></tag>
   <elementGuidId>7fd011d1-07ef-4686-b91f-033b3d9f2338</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;listVacantPsychometricTestInvolve\&quot;: [],\n    \&quot;newQuestions\&quot;: [\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO\&quot;,\n            \&quot;question\&quot;: \&quot;¿Cuál es tu experiencia con el desarrollo de aplicaciones móviles nativas? #ExperienciaAppsMóviles\&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO\&quot;,\n            \&quot;question\&quot;: \&quot;Describe cómo has implementado medidas de seguridad en tus proyectos de desarrollo. #SeguridadProyectosDev\&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO\&quot;,\n            \&quot;question\&quot;: \&quot;¿Cómo abordarías la optimización del rendimiento de una aplicación web? #OptimizaciónRendimientoWeb\&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO\&quot;,\n            \&quot;question\&quot;: \&quot;¿Cuál es tu enfoque para la resolución de problemas en el desarrollo de software? #EnfoqueResoluciónProblemasDev\&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO_S\&quot;,\n            \&quot;question\&quot;: \&quot;¿Cuál es la diferencia entre listas y data frames en R y cuándo utilizarías cada uno? \&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO_S\&quot;,\n            \&quot;question\&quot;: \&quot;¿Explícame cómo funcionan los loops en R y cuál es una situación en la que sería más adecuado utilizar un loop en lugar de aplicar una función vectorizada? \&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO_S\&quot;,\n            \&quot;question\&quot;: \&quot;¿Puedes describir cómo R gestiona los missing values en los marcos de datos y qué funciones o técnicas utilizarías para manejarlos? \&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        },\n        {\n            \&quot;exclud\&quot;: false,\n            \&quot;type\&quot;: \&quot;VIDEO_S\&quot;,\n            \&quot;question\&quot;: \&quot;¿Cuál es la función de la función \u0027apply\u0027 en R y cómo la has utilizado en tus proyectos anteriores?\&quot;,\n            \&quot;typeQuestion\&quot;: \&quot;CERRADA\&quot;,\n            \&quot;isArmed\&quot;: false\n        }\n    ]\n}&quot;,
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
      <webElementGuid>39f5c6a3-3ba1-4e4e-b3bc-2a3b1487ada5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenReclu}</value>
      <webElementGuid>fa80a88e-daea-4a94-8504-2adc2767357b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/user/vacant/step5/${GlobalVariable.vacanteid}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>65d1750e-0e9a-4def-b8e2-38dd4d157353</id>
      <masked>false</masked>
      <name>url</name>
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
