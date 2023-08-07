<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>vacante2</name>
   <tag></tag>
   <elementGuidId>1431dca5-a039-44ca-b224-d02fd01bfec3</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/mission\&quot;,\n        \&quot;value\&quot;: \&quot;Crear soluciones innovadoras y eficientes a través de la programación, para satisfacer las necesidades tecnológicas y contribuir al éxito del equipo y la empresa.\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/functions\&quot;,\n        \&quot;value\&quot;: \&quot;* Escuchar y comprender las necesidades y requerimientos del cliente para el desarrollo de software. \\n*  Diseñar y desarrollar soluciones tecnológicas eficientes, utilizando lenguajes de programación y herramientas adecuadas. \\n*  Realizar pruebas rigurosas para asegurar la calidad del software desarrollado.\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/typePositionId\&quot;,\n        \&quot;value\&quot;: \&quot;40288086796be11e01796c18e66f0064\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/contractId\&quot;,\n        \&quot;value\&quot;: \&quot;40288088798adb5701798b18e4fb0000\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/peopleCharge\&quot;,\n        \&quot;value\&quot;: \&quot;3\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/schedule\&quot;,\n        \&quot;value\&quot;: \&quot;Lunes a viernes de 8:00 a 17:00 hrs\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/allNationality\&quot;,\n        \&quot;value\&quot;: \&quot;true\&quot;\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/workingDay\&quot;,\n        \&quot;value\&quot;: {\n            \&quot;catalogSystemId\&quot;: \&quot;0000000088d5e9020188da01e8630000\&quot;,\n            \&quot;name\&quot;: \&quot;Tiempo completo\&quot;,\n            \&quot;type\&quot;: \&quot;workingDay\&quot;,\n            \&quot;status\&quot;: true,\n            \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n        }\n    },\n    {\n        \&quot;op\&quot;: \&quot;replace\&quot;,\n        \&quot;path\&quot;: \&quot;/steps\&quot;,\n        \&quot;value\&quot;: \&quot;2\&quot;\n    }\n]&quot;,
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
      <webElementGuid>abd380f4-a869-4d79-8e58-a51afe35bd1a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenReclu}</value>
      <webElementGuid>07d5d204-1918-4646-b236-853d41919b40</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/user/vacant/${GlobalVariable.vacanteid}</restUrl>
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
      <id>97a9d539-2a73-4f22-b7b8-732886d7c5c0</id>
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
