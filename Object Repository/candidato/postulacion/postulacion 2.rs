<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>postulacion 2</name>
   <tag></tag>
   <elementGuidId>09d28d00-363b-4019-bebd-20f9610c26df</elementGuidId>
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
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n    {\n        \&quot;question\&quot;: {\n            \&quot;questionId\&quot;: \&quot;${GlobalVariable.questionId0}\&quot;\n        },\n        \&quot;binaryAnswer\&quot;: null,\n        \&quot;levelHardSkill\&quot;: null,\n        \&quot;openAnswer\&quot;: null,\n        \&quot;yearsExperienceAnswer\&quot;: 12\n    },\n    {\n        \&quot;question\&quot;: {\n            \&quot;questionId\&quot;: \&quot;${GlobalVariable.questionId1}\&quot;\n        },\n        \&quot;binaryAnswer\&quot;: null,\n        \&quot;levelHardSkill\&quot;: null,\n        \&quot;openAnswer\&quot;: null,\n        \&quot;yearsExperienceAnswer\&quot;: 12\n    },\n    {\n        \&quot;question\&quot;: {\n            \&quot;questionId\&quot;: \&quot;${GlobalVariable.questionId2}\&quot;\n        },\n        \&quot;binaryAnswer\&quot;: false,\n        \&quot;levelHardSkill\&quot;: null,\n        \&quot;openAnswer\&quot;: null,\n        \&quot;yearsExperienceAnswer\&quot;: null\n    }\n]&quot;,
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
      <webElementGuid>561a6fbd-6f6c-4cfb-8cdd-afbece928ffc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TokenCand}</value>
      <webElementGuid>a8cab71e-3471-451b-b63e-f40df8ddcab1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/candidate/candidate/postulation/answer?processId=${GlobalVariable.postulacionId}&amp;questionnaire=EXPERIENCE</restUrl>
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
      <id>a18f383c-6b52-4c23-9aaf-405ae33a36ac</id>
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
