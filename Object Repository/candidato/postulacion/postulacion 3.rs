<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>postulacion 3</name>
   <tag></tag>
   <elementGuidId>f5d7a826-1e23-4ebc-8a6b-83d13729d170</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.TokenCand}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n    {\n        \&quot;question\&quot;: {\n            \&quot;questionId\&quot;: \&quot;${GlobalVariable.questionId0}\&quot;\n        },\n        \&quot;binaryAnswer\&quot;: null,\n        \&quot;levelHardSkill\&quot;: \&quot;EXPERTO\&quot;,\n        \&quot;openAnswer\&quot;: null,\n        \&quot;yearsExperienceAnswer\&quot;: null\n    },\n    {\n        \&quot;question\&quot;: {\n            \&quot;questionId\&quot;: \&quot;${GlobalVariable.questionId1}\&quot;\n        },\n        \&quot;binaryAnswer\&quot;: null,\n        \&quot;levelHardSkill\&quot;: null,\n        \&quot;openAnswer\&quot;: \&quot;se manejar toda la paquetería de office con un alto nivel\&quot;,\n        \&quot;yearsExperienceAnswer\&quot;: null\n    }\n]&quot;,
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
      <webElementGuid>d0c60027-1553-4e9b-89f9-20875ecea02a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenCand}</value>
      <webElementGuid>68642d0e-9f2a-46cc-b308-72d6a5e2dafd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/user/candidate/postulation/answer?processId=${GlobalVariable.postulacionId}&amp;questionnaire=HARD_SKILL</restUrl>
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
      <id>17c7e262-a690-4d1e-b254-979b9b3a9a2d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.habilidad_dura</defaultValue>
      <description></description>
      <id>90691038-abde-4c56-8029-91b8b58f3a94</id>
      <masked>false</masked>
      <name>habilidad_dura</name>
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
