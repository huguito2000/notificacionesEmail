<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>vacantes activas</name>
   <tag></tag>
   <elementGuidId>31088358-b0b6-4c71-b693-e0cbf6b0bad2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJqdGkiOiIwMDAwMDAwMDg4YjU1YWE4MDE4OGM1OGJjZDcxMDAyZiIsInN1YiI6Imh1Z3VpdG8ucmVjbHV0YWRvckB5b3BtYWlsLmNvbSIsImF1dGhvcml0aWVzIjpbIlJFQ1JVSVRFUl9BRE1JTiJdLCJpZCI6IjAwMDAwMDAwODhiNTVhYTgwMTg4YzU4YmNkNzEwMDJmIiwidXNlclJvbCI6IlJFQ1JVSVRFUl9BRE1JTiIsImVtYWlsVXNlciI6Imh1Z3VpdG8ucmVjbHV0YWRvckB5b3BtYWlsLmNvbSIsImtleVN5c3RlbSI6Ik1FWCIsImlhdCI6MTY5MTQyNDIyMywiZXhwIjoxNjkxNDM1MDIzfQ.hwkyvh7vYeNtTqH_8G6YeRmaVLhHbBuQSS2lbXRGPgLqs4cZcI0wQXCgsDwbbnxGFeEK1sDTskEgkYEr40pSGw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJqdGkiOiIwMDAwMDAwMDg4YjU1YWE4MDE4OGM1OGJjZDcxMDAyZiIsInN1YiI6Imh1Z3VpdG8ucmVjbHV0YWRvckB5b3BtYWlsLmNvbSIsImF1dGhvcml0aWVzIjpbIlJFQ1JVSVRFUl9BRE1JTiJdLCJpZCI6IjAwMDAwMDAwODhiNTVhYTgwMTg4YzU4YmNkNzEwMDJmIiwidXNlclJvbCI6IlJFQ1JVSVRFUl9BRE1JTiIsImVtYWlsVXNlciI6Imh1Z3VpdG8ucmVjbHV0YWRvckB5b3BtYWlsLmNvbSIsImtleVN5c3RlbSI6Ik1FWCIsImlhdCI6MTY5MTQyNDIyMywiZXhwIjoxNjkxNDM1MDIzfQ.hwkyvh7vYeNtTqH_8G6YeRmaVLhHbBuQSS2lbXRGPgLqs4cZcI0wQXCgsDwbbnxGFeEK1sDTskEgkYEr40pSGw</value>
      <webElementGuid>851f3604-d138-48ae-8003-e9ac31a62795</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/user/vacant/admin?pageSize=5&amp;pageNumber=0&amp;sortBy=publicationDate&amp;sortDirection=DESC&amp;status=ACTIVA</restUrl>
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
      <id>6a8350a3-840f-4efb-acad-8bc47e26539e</id>
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
